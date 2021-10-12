#pragma once
#include "spdlog/spdlog.h"

#define DEFAULT_LOGGER_NAME "com.apaxson.indiemotion"

#if defined(LOG_PLATFORM_WINDOWS)
#define LOG_BREAK __debugbreak();
#elif defined(LOG_PLATFORM_MAC)
#define LOG_BREAK __builtin_debugtrap();
#else
#define LOG_BREAK __builtin_trap();
#endif

#ifndef LOG_CONFIG_RELEASE
#define LOG_TRACE(...)                                        \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)          \
    {                                                         \
        spdlog::get(DEFAULT_LOGGER_NAME)->trace(__VA_ARGS__); \
    }
#define LOG_DEBUG(...)                                        \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)          \
    {                                                         \
        spdlog::get(DEFAULT_LOGGER_NAME)->debug(__VA_ARGS__); \
    }
#define LOG_INFO(...)                                        \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)         \
    {                                                        \
        spdlog::get(DEFAULT_LOGGER_NAME)->info(__VA_ARGS__); \
    }
#define LOG_WARN(...)                                        \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)         \
    {                                                        \
        spdlog::get(DEFAULT_LOGGER_NAME)->warn(__VA_ARGS__); \
    }
#define LOG_ERROR(...)                                        \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)          \
    {                                                         \
        spdlog::get(DEFAULT_LOGGER_NAME)->error(__VA_ARGS__); \
    }
#define LOG_FATAL(...)                                           \
    if (spdlog::get(DEFAULT_LOGGER_NAME) != nullptr)             \
    {                                                            \
        spdlog::get(DEFAULT_LOGGER_NAME)->critical(__VA_ARGS__); \
    }
#define LOG_ASSERT(x, msg)                                                                         \
    if ((x))                                                                                       \
    {                                                                                              \
    }                                                                                              \
    else                                                                                           \
    {                                                                                              \
        LOG_FATAL("ASSERT - {}\n\t{}\n\tin file: {}\n\ton line: {}", #x, msg, __FILE__, __LINE__); \
        LOG_BREAK                                                                                  \
    }
#else
// Disable logging for release builds
#define LOG_TRACE(...) (void)0
#define LOG_DEBUG(...) (void)0
#define LOG_INFO(...) (void)0
#define LOG_WARN(...) (void)0
#define LOG_ERROR(...) (void)0
#define LOG_FATAL(...) (void)0
#define LOG_ASSERT(x, msg) (void)0
#endif