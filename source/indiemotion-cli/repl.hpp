#pragma once
#include <iostream>
#include <string>

#include <indiemotion/_common.hpp>
#include <replxx.h>

class ReplWriter
{
private:
    std::weak_ptr<replxx::Replxx> _m_repl;

public:
    // Default Constructor
    ReplWriter(
        std::shared_ptr<replxx::Replxx> repl) : _m_repl(repl){};

    // Copy the resource (copy constructor)
    ReplWriter(const ReplWriter &rhs)
    {
        _m_repl = rhs._m_repl;
    }

    // Transfer Ownership (move constructor)
    ReplWriter(ReplWriter &&rhs) noexcept
    {
        _m_repl = rhs._m_repl;
    }

    // Make type `std::swap`able
    friend void swap(ReplWriter &a, ReplWriter &b) noexcept
    {
        a.swap(b);
    }

    // Destructor
    ~ReplWriter() {}

    // Assignment by Value
    ReplWriter &operator=(ReplWriter copy)
    {
        copy.swap(*this);
        return *this;
    }

    void swap(ReplWriter &rhs) noexcept
    {
        using std::swap;
        swap(_m_repl, rhs._m_repl);
    }
    /**
     * @brief Write formatted string to repl output.
     * 
     * @tparam Args nothing
     * @param f format string
     * @param args Arguments to use in format string
     */
    template <typename... Args>
    void write(const char *f, Args... args)
    {
        if (auto repl = _m_repl.lock())
        {
            repl->print(f, args...);
        }
        else
        {
            std::cerr << "Writer Repl is expired.\n";
        }
    }
};

int utf8str_codepoint_len(char const *s, int utf8len)
{
    int codepointLen = 0;
    unsigned char m4 = 128 + 64 + 32 + 16;
    unsigned char m3 = 128 + 64 + 32;
    unsigned char m2 = 128 + 64;
    for (int i = 0; i < utf8len; ++i, ++codepointLen)
    {
        char c = s[i];
        if ((c & m4) == m4)
        {
            i += 3;
        }
        else if ((c & m3) == m3)
        {
            i += 2;
        }
        else if ((c & m2) == m2)
        {
            i += 1;
        }
    }
    return (codepointLen);
}

int context_len(char const *prefix)
{
    char const wb[] = " \t\n\r\v\f-=+*&^%$#@!,./?<>;:`~'\"[]{}()\\|";
    int i = (int)strlen(prefix) - 1;
    int cl = 0;
    while (i >= 0)
    {
        if (strchr(wb, prefix[i]) != NULL)
        {
            break;
        }
        ++cl;
        --i;
    }
    return (cl);
}

replxx::Replxx::completions_t
hook_completion(std::string const &context, int &contextLen, std::vector<std::string> const &examples)
{
    replxx::Replxx::completions_t completions;
    int utf8ContextLen(context_len(context.c_str()));
    int prefixLen(static_cast<int>(context.length()) - utf8ContextLen);
    if ((prefixLen > 0) && (context[prefixLen - 1] == '\\'))
    {
        --prefixLen;
        ++utf8ContextLen;
    }
    contextLen = utf8str_codepoint_len(context.c_str() + prefixLen, utf8ContextLen);

    std::string prefix{context.substr(prefixLen)};
    if (prefix == "\\pi")
    {
        completions.push_back("π");
    }
    else
    {
        for (auto const &e : examples)
        {
            if (e.compare(0, prefix.size(), prefix) == 0)
            {
                replxx::Replxx::Color c(replxx::Replxx::Color::DEFAULT);
                if (e.find("brightred") != std::string::npos)
                {
                    c = replxx::Replxx::Color::BRIGHTRED;
                }
                else if (e.find("red") != std::string::npos)
                {
                    c = replxx::Replxx::Color::RED;
                }
                completions.emplace_back(e.c_str(), c);
            }
        }
    }

    return completions;
}

class ReplCore
{
private:
    std::shared_ptr<replxx::Replxx> _m_repl;
    std::shared_ptr<ReplWriter> _m_writer;
    std::string _m_history_file;
    std::string _m_prompt;

public:
    // Default Constructor
    ReplCore()
    {
        _m_prompt = "\x1b[1;32mindiemotion\x1b[0m> ";
        _m_repl = std::make_shared<replxx::Replxx>();
        _m_writer = std::make_shared<ReplWriter>(_m_repl);

        _m_repl->install_window_change_handler();

        // load the history file if it exists
        _m_history_file = "$/.indiemotion_history";
        _m_repl->history_load(_m_history_file);
    };

    // Copy the resource (copy constructor)
    ReplCore(const ReplCore &rhs) = delete;

    // Transfer Ownership (move constructor)
    ReplCore(ReplCore &&rhs) noexcept
    {
        _m_prompt = std::exchange(rhs._m_prompt, "");
        _m_repl = std::exchange(rhs._m_repl, nullptr);
        _m_history_file = std::exchange(rhs._m_history_file, nullptr);
    }

    // Make type `std::swap`able
    friend void swap(ReplCore &a, ReplCore &b) noexcept
    {
        a.swap(b);
    }

    // Destructor
    ~ReplCore() {}

    // Assignment by Value
    ReplCore &operator=(ReplCore copy)
    {
        copy.swap(*this);
        return *this;
    }

    void swap(ReplCore &rhs) noexcept
    {
        using std::swap;
        swap(_m_prompt, rhs._m_prompt);
        swap(_m_repl, rhs._m_repl);
        swap(_m_history_file, rhs._m_history_file);
    }

    void start()
    {
        // words to be completed
        std::vector<std::string> examples{
            ".help",
            ".history",
            ".quit",
            ".exit",
            ".clear",
            ".prompt ",
            "hello",
            "world",
            "db",
            "data",
            "drive",
            "print",
            "put",
            "my_custom_example",
        };

        using namespace std::placeholders;
        _m_repl->set_completion_callback(std::bind(&hook_completion, _1, _2, cref(examples)));

        for (;;)
        {
            // display the prompt and retrieve input from the user
            char const *cinput{nullptr};

            do
            {
                cinput = _m_repl->input(_m_prompt);
            } while ((cinput == nullptr) && (errno == EAGAIN));

            if (cinput == nullptr)
            {
                break;
            }
            // change cinput into a std::string
            // easier to manipulate
            std::string input{cinput};

            if (input.empty())
            {
                // user hit enter on an empty line
                continue;
            }
            else if (input.compare(0, 5, ".quit") == 0 || input.compare(0, 5, ".exit") == 0)
            {
                // exit the repl
                _m_repl->history_add(input);
                break;
            }
            else
            {
                // default action
                // echo the input
                _m_writer->write("write: %s\n", input.c_str());
                // _m_repl->print("input: %s\n", input.c_str());
                _m_repl->history_add(input);
                continue;
            }

            // save the history
            _m_repl->history_sync(_m_history_file);

            std::cout << "\nExiting...\n";
        }
    }

    std::shared_ptr<ReplWriter> get_writer() noexcept
    {
        return _m_writer;
    }
};
