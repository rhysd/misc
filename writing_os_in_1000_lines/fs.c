#include "kernel.h"

#define SECTOR_SIZE 512
#define VIRTQ_ENTRY_NUM 16
#define VIRTIO_DEVICE_BLK 2
#define VIRTIO_REG_MAGIC 0x00
#define VIRTIO_REG_VERSION 0x04
#define VIRTIO_REG_DEVICE_ID 0x08
#define VIRTIO_REG_QUEUE_SEL 0x30
#define VIRTIO_REG_QUEUE_NUM_MAX 0x34
#define VIRTIO_REG_QUEUE_NUM 0x38
#define VIRTIO_REG_QUEUE_ALIGN 0x3c
#define VIRTIO_REG_QUEUE_PFN 0x40
#define VIRTIO_REG_QUEUE_READY 0x44
#define VIRTIO_REG_QUEUE_NOTIFY 0x50
#define VIRTIO_REG_DEVICE_STATUS 0x70
#define VIRTIO_REG_DEVICE_CONFIG 0x100
#define VIRTIO_STATUS_ACK 1
#define VIRTIO_STATUS_DRIVER 2
#define VIRTIO_STATUS_DRIVER_OK 4
#define VIRTIO_STATUS_FEAT_OK 8
#define VIRTQ_DESC_F_NEXT 1
#define VIRTQ_DESC_F_WRITE 2
#define VIRTQ_AVAIL_F_NO_INTERRUPT 1
#define VIRTIO_BLK_T_IN 0
#define VIRTIO_BLK_T_OUT 1
#define FILES_MAX 3
#define DISK_MAX_SIZE align_up(sizeof(struct file) * FILES_MAX, SECTOR_SIZE)

struct virtq_desc {
    uint64_t addr;
    uint32_t len;
    uint16_t flags;
    uint16_t next;
} __attribute__((packed));

struct virtq_avail {
    uint16_t flags;
    uint16_t index;
    uint16_t ring[VIRTQ_ENTRY_NUM];
} __attribute__((packed));

struct virtq_used_elem {
    uint32_t id;
    uint32_t len;
} __attribute__((packed));

struct virtq_used {
    uint16_t flags;
    uint16_t index;
    struct virtq_used_elem ring[VIRTQ_ENTRY_NUM];
} __attribute__((packed));

struct virtio_virtq {
    struct virtq_desc descs[VIRTQ_ENTRY_NUM];
    struct virtq_avail avail;
    struct virtq_used used __attribute__((aligned(PAGE_SIZE)));
    int queue_index;
    volatile uint16_t *used_index;
    uint16_t last_used_index;
} __attribute__((packed));

// 5.2.6.4 Legacy Interface: Framing Requirements
// > MUST use a single 8-byte descriptor containing type, reserved and sector, followed by descriptors
// > for data, then finally a separate 1-byte descriptor for status.
struct virtio_blk_req {
    // The first descriptior. Read-only from virtio-blk device.
    uint32_t type;
    uint32_t reserved;
    uint64_t sector;
    // The second descriptor (VIRTQ_DESC_F_WRITE).
    uint8_t data[512];
    // The third descriptor (VIRTQ_DESC_F_WRITE)
    uint8_t status;
} __attribute__((packed));

// UStar header: https://en.wikipedia.org/wiki/Tar_(computing)#UStar_format
struct tar_header {
    char name[100];
    char mode[8];
    char uid[8];
    char gid[8];
    char size[12];
    char mtime[12];
    char checksum[8];
    char type;
    char linkname[100];
    char magic[6];
    char version[2];
    char uname[32];
    char gname[32];
    char devmajor[8];
    char devminor[8];
    char prefix[155];
    char padding[12];
    char data[]; // Pointer to the data after this header
} __attribute__((packed));

static struct virtio_virtq *blk_request_vq;
static struct virtio_blk_req *blk_req;
static paddr_t blk_req_paddr;
static unsigned blk_capacity;
static struct file files[FILES_MAX];
static uint8_t disk[DISK_MAX_SIZE];

static uint32_t virtio_reg_read32(unsigned const offset) {
    return *((volatile uint32_t *)(VIRTIO_BLK_PADDR + offset));
}

static uint64_t virtio_reg_read64(unsigned const offset) {
    return *((volatile uint64_t *)(VIRTIO_BLK_PADDR + offset));
}

static void virtio_reg_write32(unsigned const offset, uint32_t const value) {
    *((volatile uint32_t *)(VIRTIO_BLK_PADDR + offset)) = value;
}

static void virtio_reg_fetch_and_or32(unsigned const offset, uint32_t const value) {
    virtio_reg_write32(offset, virtio_reg_read32(offset) | value);
}

// Initialize virtual queue
struct virtio_virtq *virtq_init(unsigned const index) {
    // 4.2.4 Legacy interface
    paddr_t virtq_paddr = alloc_pages(align_up(sizeof(struct virtio_virtq), PAGE_SIZE) / PAGE_SIZE);
    struct virtio_virtq *vq = (struct virtio_virtq *)virtq_paddr;
    vq->queue_index = index;
    vq->used_index = (volatile uint16_t *)&vq->used.index;
    // 1. Select the queue writing its index (first queue is 0) to QueueSel.
    virtio_reg_write32(VIRTIO_REG_QUEUE_SEL, index);
    // 5. Notify the device about the queue size by writing the size to QueueNum.
    virtio_reg_write32(VIRTIO_REG_QUEUE_NUM, VIRTQ_ENTRY_NUM);
    // 6. Notify the device about the used alignment by writing its value in bytes to QueueAlign.
    virtio_reg_write32(VIRTIO_REG_QUEUE_ALIGN, 0);
    // 7. Write the physical number of the first page of the queue to the QueuePFN register.
    virtio_reg_write32(VIRTIO_REG_QUEUE_PFN, virtq_paddr);
    return vq;
}

// Explanation of Virtio buffer operation: https://www.redhat.com/ja/blog/virtqueues-and-virtio-ring-how-data-travels

// Initialize virtio-blk device
void virtio_blk_init(void) {
    if (virtio_reg_read32(VIRTIO_REG_MAGIC) != 0x74726976) {
        PANIC("virtio: invalid magic value");
    }
    if (virtio_reg_read32(VIRTIO_REG_VERSION) != 1) {
        PANIC("virtio: invalid version");
    }
    if (virtio_reg_read32(VIRTIO_REG_DEVICE_ID) != VIRTIO_DEVICE_BLK) {
        PANIC("virtio: invalid device id");
    }

    // 3.1.1 Driver Requirements: Device Initialization The driver MUST follow this sequence to initialize a device:

    // 1. Reset the device.
    virtio_reg_write32(VIRTIO_REG_DEVICE_STATUS, 0);
    // 2. Set the ACKNOWLEDGE status bit: the guest OS has noticed the device.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_ACK);
    // 3. Set the DRIVER status bit.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER);
    // 5. Set the FEATURES_OK status bit.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_FEAT_OK);
    // 7. Perform device-specific setup, including discovery of virtqueues for the device
    blk_request_vq = virtq_init(0);
    // 8. Set the DRIVER_OK status bit.
    virtio_reg_write32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER_OK);

    // Get the disk space
    blk_capacity = virtio_reg_read64(VIRTIO_REG_DEVICE_CONFIG + 0) * SECTOR_SIZE;
    printf("virtio-blk: capacity is %d bytes\n", blk_capacity);

    // Allocate memory for processing device request
    blk_req_paddr = alloc_pages(align_up(sizeof(*blk_req), PAGE_SIZE) / PAGE_SIZE);
    blk_req = (struct virtio_blk_req *)blk_req_paddr;
}

// Notify virtio-blk device that there is a new request. desc_index is an index of the first descriptor
// of the new request.
static void virtq_kick(struct virtio_virtq *vq, int const desc_index) {
    vq->avail.ring[vq->avail.index % VIRTQ_ENTRY_NUM] = desc_index;
    vq->avail.index++;
    __sync_synchronize();
    virtio_reg_write32(VIRTIO_REG_QUEUE_NOTIFY, vq->queue_index);
    vq->last_used_index++;
}

// Check if there is some request being processed by the virtio-blk device.
static bool virtq_is_busy(struct virtio_virtq const *vq) {
    return vq->last_used_index != *vq->used_index;
}

// Read/Write virtio-blk device
static void read_write_disk(void *buf, unsigned const sector, int const is_write) {
    if (sector >= blk_capacity / SECTOR_SIZE) {
        printf("virtio: tried to read/write sector=%d, but capacity is %d\n", sector, blk_capacity / SECTOR_SIZE);
        return;
    }

    // Build a request following the specification of virtio-blk
    blk_req->sector = sector;
    blk_req->type = is_write ? VIRTIO_BLK_T_OUT : VIRTIO_BLK_T_IN;
    if (is_write) {
        memcpy(blk_req->data, buf, SECTOR_SIZE);
    }

    // Build 3 descriptors for virtual queue
    struct virtio_virtq *vq = blk_request_vq;

    vq->descs[0].addr = blk_req_paddr;
    // Read 3 fields of vertio_blk_req: sizeof(type) + sizeof(reserved) + sizeof(sector)
    vq->descs[0].len = sizeof(uint32_t) * 2 + sizeof(uint64_t);
    vq->descs[0].flags = VIRTQ_DESC_F_NEXT; // 2.7.6 Next Flag: Descriptor Chaining
    vq->descs[0].next = 1;

    // 2.6.13.1 Placing Buffers Into The Descriptor Table
    vq->descs[1].addr = blk_req_paddr + offsetof(struct virtio_blk_req, data);
    vq->descs[1].len = SECTOR_SIZE;
    vq->descs[1].flags = VIRTQ_DESC_F_NEXT | (is_write ? 0 : VIRTQ_DESC_F_WRITE);
    vq->descs[1].next = 2;

    // Write status field of virtio_blk_req
    vq->descs[2].addr = blk_req_paddr + offsetof(struct virtio_blk_req, status);
    vq->descs[2].len = sizeof(uint8_t);
    vq->descs[2].flags = VIRTQ_DESC_F_WRITE; // 2.7.3 Write Flag

    // Note: We always use the same descriptors for every request (vq->desc[0..2]) because we wait
    // for the request result with blocking.

    // Notify the virtio-blk device that there are some requests to process.
    virtq_kick(vq, 0);

    // Wait until virtio-blk device finishes to process the request.
    while (virtq_is_busy(vq)) {
    }

    // Check the request status. Non-zero means an error
    if (blk_req->status != 0) {
        printf("virtio: warn: failed to read/write sector=%d status=%d\n", sector, blk_req->status);
        return;
    }

    // Copy the data read from the device to the buffer.
    if (!is_write) {
        memcpy(buf, blk_req->data, SECTOR_SIZE);
    }
}

static int parse_octal(char const *oct, int const len) {
    int dec = 0;
    for (int i = 0; i < len; i++) {
        char const c = oct[i];
        if (c < '0' || '7' < c) {
            break;
        }

        dec = dec * 8 + (c - '0');
    }
    return dec;
}

void fs_init(void) {
    // Note: We don't use local variable for `disk` because it consumes stack. (The stack size is small)

    for (size_t sector = 0; sector < sizeof(disk) / SECTOR_SIZE; sector++) {
        read_write_disk(&disk[sector * SECTOR_SIZE], sector, false); // Read the file to `disk` variable
    }

    size_t offset = 0;
    for (int i = 0; i < FILES_MAX; i++) {
        struct tar_header const *header = (struct tar_header *)&disk[offset];
        if (header->name[0] == '\0') {
            break;
        }

        if (strcmp(header->magic, "ustar") != 0) {
            PANIC("invalid tar header: magic=\"%s\"", header->magic);
        }

        int const size = parse_octal(header->size, sizeof(header->size));
        // Add only normal files (type='0')
        if (header->type == '0') {
            struct file *file = &files[i];
            file->in_use = true;
            strcpy(file->name, header->name);
            memcpy(file->data, header->data, size);
            file->size = size;
            printf("file: %s, size=%d offset=%d\n", file->name, file->size, offset);
        }

        offset += align_up(sizeof(struct tar_header) + size, SECTOR_SIZE);
    }
}

static void unparse_octal(char *buf, int value) {
    int i = 0;
    do {
        buf[i++] = (value % 8) + '0';
        value /= 8;
    } while (value > 0);
}

// Write all files to the disk
void fs_flush(void) {
    memset(disk, 0, sizeof(disk));
    size_t offset = 0;
    for (int j = 0; j < FILES_MAX; j++) {
        struct file *file = &files[j];
        if (!file->in_use) {
            continue;
        }

        // Create tar header

        struct tar_header *header = (struct tar_header *)&disk[offset];
        memset(header, 0, sizeof(*header));
        strcpy(header->magic, "ustar");
        strcpy(header->mode, "000644");
        strcpy(header->version, "00");
        strcpy(header->name, file->name);
        header->type = '0';

        unparse_octal(header->size, file->size);

        int checksum = ' ' * sizeof(header->checksum);
        for (size_t i = 0; i < sizeof(struct tar_header); i++) {
            checksum += (unsigned char)disk[offset + i];
        }
        for (int i = 5; i >= 0; i--) {
            header->checksum[i] = (checksum % 8) + '0';
            checksum /= 8;
        }

        memcpy(header->data, file->data, file->size);
        offset += align_up(sizeof(struct tar_header) + file->size, SECTOR_SIZE);
    }

    // Write the entire on-memory data to the disk
    for (size_t sector = 0; sector < sizeof(disk) / SECTOR_SIZE; sector++) {
        read_write_disk(&disk[sector * SECTOR_SIZE], sector, true);
    }

    printf("wrote %d bytes to disk\n", sizeof(disk));
}

struct file *fs_lookup(char const *filename) {
    for (int i = 0; i < FILES_MAX; i++) {
        struct file *file = &files[i];
        if (strcmp(file->name, filename) == 0) {
            return file;
        }
    }
    return NULL;
}
