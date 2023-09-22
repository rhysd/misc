#include "kernel.h"
#include "common.h"

extern char __bss[], __bss_end[], __stack_top[], __free_ram[], __free_ram_end[], __kernel_base[];

// Start and end address of the raw binary shell.bin.o
extern char _binary_shell_bin_start[], _binary_shell_bin_size[];

struct process procs[PROCS_MAX]; // Note: All elements are zero-initialized so the state is PROC_STATE_UNUSED

// Chapter 3. Binary Encoding
//   fid = SBI function ID, eid = SBI extension ID
struct sbiret sbi_call(long arg0, long arg1, long arg2, long arg3, long arg4,
                       long arg5, long fid, long eid) {
    register long a0 __asm__("a0") = arg0;
    register long a1 __asm__("a1") = arg1;
    register long a2 __asm__("a2") = arg2;
    register long a3 __asm__("a3") = arg3;
    register long a4 __asm__("a4") = arg4;
    register long a5 __asm__("a5") = arg5;
    register long a6 __asm__("a6") = fid;
    register long a7 __asm__("a7") = eid;

    __asm__ __volatile__("ecall"
                         : "=r"(a0), "=r"(a1)
                         : "r"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5),
                           "r"(a6), "r"(a7)
                         : "memory");
    struct sbiret ret;
    ret.error = a0;
    ret.value = a1;
    return ret;
}

// 5.2. Extension: Console Putchar (EID #0x01)
void putchar(char ch) {
    sbi_call(ch, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
}

// 5.3. Extension: Console Getchar (EID #0x02)
long getchar() {
    struct sbiret ret = sbi_call(0, 0, 0, 0, 0, 0, 0, 2 /* Console Getchar */);
    return ret.error;
}

// Exception handler.
// - Save and restore registers using stack. The fp registers are not needed to be saved because they are not used by kernel
// - The sscratch register is used to save/restore stack pointer
__attribute__((naked))
__attribute__((aligned(4))) // because stvec contains the execution mode in the lower 2 bits
void
exception_handler(void) {
    __asm__ __volatile__(
        // Get the bottom address of the current process's kernel stack from sscratch register which was saved to
        // sscratch register in yield() function. Instead, the current stack pointer (where the exception happened)
        // is saved to sscratch register.
        //   tmp = sp; sp = sscratch; sscratch = tmp;
        "csrrw sp, sscratch, sp\n"

        // Save registers before calling trap handler
        "addi sp, sp, -4 * 31\n"
        "sw ra,  4 * 0(sp)\n"
        "sw gp,  4 * 1(sp)\n"
        "sw tp,  4 * 2(sp)\n"
        "sw t0,  4 * 3(sp)\n"
        "sw t1,  4 * 4(sp)\n"
        "sw t2,  4 * 5(sp)\n"
        "sw t3,  4 * 6(sp)\n"
        "sw t4,  4 * 7(sp)\n"
        "sw t5,  4 * 8(sp)\n"
        "sw t6,  4 * 9(sp)\n"
        "sw a0,  4 * 10(sp)\n"
        "sw a1,  4 * 11(sp)\n"
        "sw a2,  4 * 12(sp)\n"
        "sw a3,  4 * 13(sp)\n"
        "sw a4,  4 * 14(sp)\n"
        "sw a5,  4 * 15(sp)\n"
        "sw a6,  4 * 16(sp)\n"
        "sw a7,  4 * 17(sp)\n"
        "sw s0,  4 * 18(sp)\n"
        "sw s1,  4 * 19(sp)\n"
        "sw s2,  4 * 20(sp)\n"
        "sw s3,  4 * 21(sp)\n"
        "sw s4,  4 * 22(sp)\n"
        "sw s5,  4 * 23(sp)\n"
        "sw s6,  4 * 24(sp)\n"
        "sw s7,  4 * 25(sp)\n"
        "sw s8,  4 * 26(sp)\n"
        "sw s9,  4 * 27(sp)\n"
        "sw s10, 4 * 28(sp)\n"
        "sw s11, 4 * 29(sp)\n"

        // Push the sp (where the exception happened) to stack
        "csrr a0, sscratch\n"
        "sw a0, 4 * 30(sp)\n"

        // Reset the kernel stack to sscratch register again
        "addi a0, sp, 4 * 31\n"
        "csrw sscratch, a0\n"

        "mv a0, sp\n"
        "call handle_trap\n"

        // Restore registers after calling trap handler
        "lw ra,  4 * 0(sp)\n"
        "lw gp,  4 * 1(sp)\n"
        "lw tp,  4 * 2(sp)\n"
        "lw t0,  4 * 3(sp)\n"
        "lw t1,  4 * 4(sp)\n"
        "lw t2,  4 * 5(sp)\n"
        "lw t3,  4 * 6(sp)\n"
        "lw t4,  4 * 7(sp)\n"
        "lw t5,  4 * 8(sp)\n"
        "lw t6,  4 * 9(sp)\n"
        "lw a0,  4 * 10(sp)\n"
        "lw a1,  4 * 11(sp)\n"
        "lw a2,  4 * 12(sp)\n"
        "lw a3,  4 * 13(sp)\n"
        "lw a4,  4 * 14(sp)\n"
        "lw a5,  4 * 15(sp)\n"
        "lw a6,  4 * 16(sp)\n"
        "lw a7,  4 * 17(sp)\n"
        "lw s0,  4 * 18(sp)\n"
        "lw s1,  4 * 19(sp)\n"
        "lw s2,  4 * 20(sp)\n"
        "lw s3,  4 * 21(sp)\n"
        "lw s4,  4 * 22(sp)\n"
        "lw s5,  4 * 23(sp)\n"
        "lw s6,  4 * 24(sp)\n"
        "lw s7,  4 * 25(sp)\n"
        "lw s8,  4 * 26(sp)\n"
        "lw s9,  4 * 27(sp)\n"
        "lw s10, 4 * 28(sp)\n"
        "lw s11, 4 * 29(sp)\n"
        "lw sp,  4 * 30(sp)\n"
        "sret\n");
}

paddr_t alloc_pages(uint32_t const n) {
    static paddr_t next_paddr = (paddr_t)__free_ram;
    paddr_t paddr = next_paddr;
    next_paddr += n * PAGE_SIZE;

    if (next_paddr > (paddr_t)__free_ram_end) {
        PANIC("out of memory");
    }

    memset((void *)paddr, 0, n * PAGE_SIZE);
    return paddr;
}

// Register the mapping between the virtual address and the physical address in the page table
void map_page(uint32_t *table1, uint32_t vaddr, paddr_t paddr, uint32_t flags) {
    // Sv32 is a two-phase lookup table. It means a tree structure where depth is 3.
    // Depth-1 is VPN[1], depth-2 is VPN[0], depth-3 is the page offset in the 4KiB page.
    //
    // Example of Sv32 page mapping
    // ┌──────────────────────────────────┐
    // │             8001123              │ Virtual address
    // └──────────────────────────────────┘
    // ┌──────────┬──────────┬────────────┐
    // │0000100000│0000000001│000100100011│ Page table entry
    // └──────────┴──────────┴────────────┘
    //    VPN[1]     VPN[0]     page offset
    //   (10 bits)  (10 bits)  (12 bits)
    //     =32        =1         =0x123

    if (!is_aligned(vaddr, PAGE_SIZE)) {
        PANIC("unaligned vaddr %x", vaddr);
    }

    if (!is_aligned(paddr, PAGE_SIZE)) {
        PANIC("unaligned paddr %x", paddr);
    }

    uint32_t const vpn1 = (vaddr >> (10 + 12)) & 0x3ff; // Extract the first page table index
    if ((table1[vpn1] & PAGE_V) == 0) {
        // The second page table does not exist. Create it.
        uint32_t const pt_paddr = alloc_pages(1);
        table1[vpn1] = ((pt_paddr / PAGE_SIZE) << 10) | PAGE_V;
    }

    // Insert the new page entry to the second page table
    uint32_t const vpn0 = (vaddr >> 12) & 0x3ff; // Extract the second page table index
    uint32_t *table0 = (uint32_t *)((table1[vpn1] >> 10) * PAGE_SIZE);
    table0[vpn0] = ((paddr / PAGE_SIZE) << 10) | flags | PAGE_V;
}

__attribute__((naked)) void switch_context(uint32_t *prev_sp, uint32_t *next_sp) {
    __asm__ __volatile__(
        // Save ra and saved registers on current process's stack
        "addi sp, sp, -13 * 4\n"
        "sw ra,  0  * 4(sp)\n"
        "sw s0,  1  * 4(sp)\n"
        "sw s1,  2  * 4(sp)\n"
        "sw s2,  3  * 4(sp)\n"
        "sw s3,  4  * 4(sp)\n"
        "sw s4,  5  * 4(sp)\n"
        "sw s5,  6  * 4(sp)\n"
        "sw s6,  7  * 4(sp)\n"
        "sw s7,  8  * 4(sp)\n"
        "sw s8,  9  * 4(sp)\n"
        "sw s9,  10 * 4(sp)\n"
        "sw s10, 11 * 4(sp)\n"
        "sw s11, 12 * 4(sp)\n"
        "sw sp, (a0)\n" // Save the current stack pointer to `prev_sp` (=a0)

        // Restore ra and saved registers saved on next process's stack
        "lw sp, (a1)\n" // Load the current stack pointer from `next_sp` (=a1)
        "lw ra,  0  * 4(sp)\n"
        "lw s0,  1  * 4(sp)\n"
        "lw s1,  2  * 4(sp)\n"
        "lw s2,  3  * 4(sp)\n"
        "lw s3,  4  * 4(sp)\n"
        "lw s4,  5  * 4(sp)\n"
        "lw s5,  6  * 4(sp)\n"
        "lw s6,  7  * 4(sp)\n"
        "lw s7,  8  * 4(sp)\n"
        "lw s8,  9  * 4(sp)\n"
        "lw s9,  10 * 4(sp)\n"
        "lw s10, 11 * 4(sp)\n"
        "lw s11, 12 * 4(sp)\n"
        "addi sp, sp, 13 * 4\n"
        "ret\n" // Return to next process's ra
    );
}

__attribute__((naked)) void user_entry(void) {
    __asm__ __volatile__(
        // PC after switching to U-mode
        "csrw sepc, %[sepc]\n"
        // Set SPIE bit. This enables interrupts in U-mode so that trap handler in stvec is called
        // same as exceptions.
        // Note: Our implementation does not use interrupts actually (use polling instead) so this
        // flag is actually unnecessary. But it is a good idea to always enable this bit so that
        // interrupts are not ignored.
        "csrw sstatus, %[sstatus]\n"
        // Switch from S-mode to U-mode
        "sret\n"
        :
        : [sepc] "r"(USER_BASE_ADDR),
          [sstatus] "r"(SSTATUS_SPIE | SSTATUS_SUM));
}

struct process *create_process(void const *image, size_t const image_size) {
    // Search available slot
    struct process *proc = NULL;
    int i;
    for (i = 0; i < PROCS_MAX; i++) {
        if (procs[i].state == PROC_STATE_UNUSED) {
            proc = &procs[i];
            break;
        }
    }

    if (!proc) {
        PANIC("no free process slots");
    }

    uint32_t *sp = (uint32_t *)&proc->stack[sizeof(proc->stack)]; // The bottom address of stack

    // Push first saved register values so that the process can be restored by context switch
    *--sp = 0;                    // s11
    *--sp = 0;                    // s10
    *--sp = 0;                    // s9
    *--sp = 0;                    // s8
    *--sp = 0;                    // s7
    *--sp = 0;                    // s6
    *--sp = 0;                    // s5
    *--sp = 0;                    // s4
    *--sp = 0;                    // s3
    *--sp = 0;                    // s2
    *--sp = 0;                    // s1
    *--sp = 0;                    // s0
    *--sp = (uint32_t)user_entry; // ra

    // Create kernel page mapping (from __kernel_base to __free_ram_end) so that kernel can access
    // to both static area (e.g. .text) and dynamically allocated area by alloc_pages().
    uint32_t *page_table = (uint32_t *)alloc_pages(1);
    for (paddr_t paddr = (paddr_t)__kernel_base; paddr < (paddr_t)__free_ram_end; paddr += PAGE_SIZE) {
        map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
    }

    // Map addresses for virtio-blk's Memory-mapped I/O
    map_page(page_table, VIRTIO_BLK_PADDR, VIRTIO_BLK_PADDR, PAGE_R | PAGE_W);

    // Create user page mapping.
    // Note: If memcpy is not used and the image binary is mapped directly, all processes which run
    // the same binary shares the same physical address.
    for (uint32_t offset = 0; offset < image_size; offset += PAGE_SIZE) {
        paddr_t const page = alloc_pages(1);
        memcpy((void *)page, image + offset, PAGE_SIZE);
        map_page(page_table, USER_BASE_ADDR + offset, page, PAGE_U | PAGE_R | PAGE_W | PAGE_X);
    }

    proc->pid = i + 1; // PID starts from 1
    proc->state = PROC_STATE_RUNNABLE;
    proc->sp = (uint32_t)sp;
    proc->page_table = page_table;
    return proc;
}

struct process *current_proc; // The process currently running
struct process *idle_proc;    // The idle process (PID=-1) which runs when no process is runnable

void yield(void) {
    // Search the runnable process to run next
    struct process *next = idle_proc;
    for (int i = 0; i < PROCS_MAX; i++) {
        struct process *proc = &procs[(current_proc->pid + i) % PROCS_MAX];
        if (proc->state == PROC_STATE_RUNNABLE && proc->pid > 0) {
            next = proc;
            break;
        }
    }

    // Only the current process is running. No context switch is needed
    if (next == current_proc) {
        return;
    }

    __asm__ __volatile__(
        // Switch the page table by registering VPN[1] to satp register.
        // The sfence.vma instruction is a memory barrier to
        // - ensure modifying the page table finishes
        // - remove the cache of the page table entry (TLB)
        "sfence.vma\n"
        "csrw satp, %[satp]\n"
        "sfence.vma\n"
        // Save the next process's stack bottom address to sscratch register for exception handling.
        "csrw sscratch, %[sscratch]\n"
        :
        : [satp] "r"(SATP_SV32 | ((uint32_t)next->page_table / PAGE_SIZE)),
          [sscratch] "r"((uint32_t)&next->stack[sizeof(next->stack)]));

    // Switch context from the current process to the next runnable process
    struct process *prev = current_proc;
    current_proc = next;
    switch_context(&prev->sp, &next->sp);
}

uint8_t virtio_reg_read8(unsigned const offset) {
    return *((volatile uint8_t *)(VIRTIO_BLK_PADDR + offset));
}

uint32_t virtio_reg_read32(unsigned const offset) {
    return *((volatile uint32_t *)(VIRTIO_BLK_PADDR + offset));
}

uint64_t virtio_reg_read64(unsigned const offset) {
    return *((volatile uint64_t *)(VIRTIO_BLK_PADDR + offset));
}

void virtio_reg_write32(unsigned const offset, uint32_t const value) {
    *((volatile uint32_t *)(VIRTIO_BLK_PADDR + offset)) = value;
}

void virtio_reg_fetch_and_or32(unsigned const offset, uint32_t const value) {
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

struct virtio_virtq *blk_request_vq;
struct virtio_blk_req *blk_req;
paddr_t blk_req_paddr;
unsigned blk_capacity;

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
void virtq_kick(struct virtio_virtq *vq, int const desc_index) {
    vq->avail.ring[vq->avail.index % VIRTQ_ENTRY_NUM] = desc_index;
    vq->avail.index++;
    __sync_synchronize();
    virtio_reg_write32(VIRTIO_REG_QUEUE_NOTIFY, vq->queue_index);
    vq->last_used_index++;
}

// Check if there is some request being processed by the virtio-blk device.
bool virtq_is_busy(struct virtio_virtq const *vq) {
    return vq->last_used_index != *vq->used_index;
}

// Read/Write virtio-blk device
void read_write_disk(void *buf, unsigned const sector, int const is_write) {
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

struct file files[FILES_MAX];
uint8_t disk[DISK_MAX_SIZE];

int parse_octal(char const *oct, int const len) {
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

void unparse_octal(char *buf, int value) {
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

void handle_syscall(struct trap_frame *f) {
    switch (f->a3) {
    case SYSCALL_PUTCHAR:
        putchar(f->a0);
        break;
    case SYSCALL_GETCHAR:
        for (;;) {
            long const ch = getchar();
            if (ch >= 0) {
                // When some character is returned, write it to a0 and return from syscall
                f->a0 = ch;
                break;
            }
            // When return value is -1, it means that no character was input. Try getchar() again.
            // However, busy loop stops entire kernel and prevents all processes execution.
            // To avoid it, yield the execution to some runnable process then try getchar() later.
            yield();
        }
        break;
    case SYSCALL_EXIT:
        printf("process %d exited\n", current_proc->pid);
        current_proc->state = PROC_STATE_EXITED;
        // TODO: Release process struct instance (kernel stack, page table, ...) since it will no longer used.
        yield();
        PANIC("unreachable");
        break;
    case SYSCALL_READFILE:
    case SYSCALL_WRITEFILE: {
        char const *filename = (char const *)f->a0;
        char *buf = (char *)f->a1;
        int len = f->a2;
        struct file *file = fs_lookup(filename);
        if (!file) {
            printf("file not found: %s\n", filename);
            f->a0 = -1;
            break;
        }
        if (len > (int)sizeof(file->data)) {
            len = file->size;
        }
        if (f->a3 == SYSCALL_WRITEFILE) {
            memcpy(file->data, buf, len);
            file->size = len;
            fs_flush();
        } else {
            memcpy(buf, file->data, len);
        }
        f->a0 = len; // Return write/read length
        break;
    }
    default:
        PANIC("unexpected syscall a3=%x\n", f->a3);
    }
}

void handle_trap(struct trap_frame *f) {
    uint32_t scause, stval, user_pc;
    READ_CSR(scause, scause);
    READ_CSR(stval, stval);
    READ_CSR(user_pc, sepc);

    switch (scause) {
    case SCAUSE_ECALL:
        handle_syscall(f);
        // Increment PC to the next to ecall instruction
        user_pc += 4;
        break;
    default:
        PANIC("unexpected trap scause=%x, stval=%x, sepc=%x\n", scause, stval, user_pc);
        break;
    }

    WRITE_CSR(sepc, user_pc);
}

void kernel_main(void) {
    // There is no guarantee that bootloader initializes bss with zeros
    memset(__bss, 0, (size_t)__bss_end - (size_t)__bss);

    // Set exception handler
    WRITE_CSR(stvec, (uint32_t)exception_handler);

    // Initialize virtio-blk device and read disk contents for filesystem
    virtio_blk_init();
    fs_init();

    // The idle process is a special process which runs when there is no runnable process
    current_proc = idle_proc = create_process(NULL, 0);
    idle_proc->pid = -1;

    // Run shell.bin.o
    create_process(_binary_shell_bin_start, (size_t)_binary_shell_bin_size);

    yield(); // Try to run the first runnable process
    PANIC("switched to idle process");
}

// Kernel entrypoint
__attribute__((section(".text.boot")))
__attribute__((naked)) // Do not generate prologue and epilogue
void
boot(void) {
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n"
        "j kernel_main\n"
        :
        : [stack_top] "r"(__stack_top));
}
