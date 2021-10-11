//
// Created by jonross on 10/8/21.
//

#ifndef CPP_MAPPEDATOMICS_H
#define CPP_MAPPEDATOMICS_H


#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <iostream>
#include <cstring>
#include <unistd.h>
#include <atomic>

struct MappedAtomics {
    std::atomic<long unsigned int> *clientPtr;
    std::atomic<long unsigned int> *serverPtr;

    inline long unsigned int spinUntilClientChange( long unsigned int lastValue ) {
        auto nextValue = lastValue;
        while( nextValue == lastValue ) {
            __builtin_ia32_pause();
            nextValue = this->clientPtr->load(std::memory_order_relaxed);
        }
        return nextValue;
    }

    MappedAtomics() {
        int shmFd = shm_open(
                "/spinnmem",
                O_RDWR,
                S_IWUSR | S_IRGRP | S_IWGRP
        );
        if(shmFd <= 0 ) {
            std::cout << "shm_open failed. exiting : " << std::strerror(errno) << std::endl;
            throw std::system_error(errno, std::generic_category());
        }
//        std::cout << "shm fd = " << shmFd << std::endl;
        if( ftruncate( shmFd, getpagesize() ) != 0 )  {
            std::cout << "ftruncate failed. exiting : " << std::strerror(errno) << std::endl;
            throw std::system_error(errno, std::generic_category());
        }


        void * mem_ptr = mmap(
                NULL,
                (size_t)getpagesize(),
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                shmFd,
                0
        );
        if( mem_ptr == 0  || mem_ptr == MAP_FAILED ) {
            std::cout << "mmap failed. exiting : " << std::strerror(errno) << std::endl;
            throw std::system_error(errno, std::generic_category());
        }

        clientPtr = static_cast<std::atomic<long unsigned int> *>(mem_ptr);

        auto charPtr = static_cast<char *>(mem_ptr);
        auto voidPtr = static_cast<void *>(charPtr+2048);

        serverPtr = static_cast<std::atomic<long unsigned int> *>(voidPtr);

    }

};


#endif //CPP_MAPPEDATOMICS_H
