#pragma once
#include "spdlog/details/null_mutex.h"
#include <indiemotion/common.hpp>
#include <mutex>
#include <spdlog/sinks/base_sink.h>

namespace indiemotion::logging
{
    using Logger = std::shared_ptr<spdlog::logger>;

    template <typename Mutex>
    class _ConsoleSink : public spdlog::sinks::base_sink<Mutex>
    {
    protected:
        void sink_it_(const spdlog::details::log_msg &msg) override
        {
            // log_msg is a struct containing the log entry info like level, timestamp, thread id etc.
            // msg.raw contains pre-formatted log

            // If needed (very likely but not mandatory), the sink formats the description before sending it to its final destination:
            spdlog::memory_buf_t formatted;
            spdlog::sinks::base_sink<Mutex>::formatter_->format(msg, formatted);

            if (msg.level <= spdlog::level::warn)
                std::cout << fmt::to_string(formatted);
            else
                std::cerr << fmt::to_string(formatted);
        }

        void flush_() override
        {
            std::cout << std::flush;
            std::cerr << std::flush;
        }
    };

    using ConsoleSinkMT = _ConsoleSink<std::mutex>;

    std::vector<std::string> _list_parent_names(std::string name);

	struct _settings
	{
		static spdlog::level::level_enum level;
	};
	spdlog::level::level_enum _settings::level = spdlog::level::trace;

	void configure_default_logger(std::string name)
	{
		auto consoleSink = std::make_shared<ConsoleSinkMT>();
		consoleSink->set_level(_settings::level);
		auto logger = std::make_shared<spdlog::logger>(name, consoleSink);
		logger->set_level(_settings::level);
		spdlog::register_logger(logger);
	}

	void set_global_level(spdlog::level::level_enum level)
	{
		_settings::level = level;
	}

    void init_logging()
    {
		configure_default_logger("root");
    }

    Logger get_logger(std::string name)
    {
        for (auto loggerName : _list_parent_names(name))
        {
            auto logger = spdlog::get(loggerName);
            if (logger)
                return logger;
        }

        auto logger = spdlog::get("root");
        if (logger)
        {
            return logger;
        }

        init_logging();
        return spdlog::get("root");
    }

    std::vector<std::string> _list_parent_names(std::string name)
    {

        std::string cur = "";
        std::vector<std::string> names{};
        std::string delimiter = ".";
        size_t last = 0;
        size_t next = 0;
        while ((next = name.find(delimiter, last)) != std::string::npos)
        {
            cur += name.substr(last, next - last);
            if (cur != "com")
            {
                names.push_back(cur);
            }
            cur += delimiter;
            last = next + 1;
        }
        cur += name.substr(last);
        names.push_back(cur);

        std::reverse(names.begin(), names.end());
        return names;
    }
} // namespace indiemotion
