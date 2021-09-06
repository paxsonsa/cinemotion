
#include <iostream>

#include <replxx.h>

#include <indiemotion/_common.hpp>

class ReplCore
{
private:
    std::unique_ptr<replxx::Replxx> _m_repl;
    std::string _m_history_file;
    std::string _m_prompt;

public:
    // Default Constructor
    ReplCore()
    {
        _m_prompt = "\x1b[1;32mindiemotion\x1b[0m> ";
        _m_repl = std::make_unique<replxx::Replxx>();
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
                _m_repl->print("input: %s\n", input.c_str());
                _m_repl->history_add(input);
                continue;
            }

            // save the history
            _m_repl->history_sync(_m_history_file);

            std::cout << "\nExiting...\n";
        }
    }
};
