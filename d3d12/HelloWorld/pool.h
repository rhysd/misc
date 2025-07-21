#pragma once

#include <cassert>
#include <unordered_map>
#include <vector>

template <class T>
class Pool {
    std::vector<T> buf_; // Ensure this vector will never reallocate memory
    std::unordered_map<T *, size_t> active_;
    size_t cur_;

    Pool(Pool const &) = delete;
    Pool &operator=(Pool &) = delete;

  public:
    explicit Pool(size_t const count) : buf_(count, T{}), active_(), cur_(0) {
        assert(count > 0);
    }
    Pool(Pool &&) = default;
    ~Pool() {}

    T *alloc() {
        return alloc_impl().first;
    }

    template <class F>
    T *alloc_with(F init) {
        auto const ret = alloc_impl();
        if (ret.first != nullptr) {
            init(ret.first, ret.second);
        }
        return ret.first;
    }

    void dealloc(T *ptr) {
        assert(ptr != nullptr);
        active_.erase(ptr);
    }

    size_t capacity() {
        return buf_.size();
    }

    size_t size() {
        return active_.size();
    }

    size_t available() {
        return capacity() - size();
    }

  private:
    std::pair<T *, size_t> alloc_impl() {
        auto const prev = cur_;
        while (true) {
            auto const ptr = &buf_[cur_];
            auto const it = active_.find(ptr);
            if (it == active_.end()) {
                auto const idx = cur_++;
                active_.emplace(ptr, idx);
                return {ptr, idx};
            }
            cur_ = (cur_ + 1) % buf_.size();
            if (prev == cur_) {
                return {nullptr, 0}; // All slots are in use
            }
        }
    }
};
