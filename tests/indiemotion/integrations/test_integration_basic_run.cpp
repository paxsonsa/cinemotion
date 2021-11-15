#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <fstream>
#include <thread>
#include <sstream>

#include <doctest.h>


// TODO move into 'testing' include folder
#include <configure.hpp>
#include <fixtures/playbook.hpp>
#include <fixtures/delegate_helpers.hpp>
#include <fixtures/json_helpers.hpp>


TEST_CASE("Execute Basic Start Up Integration")
{
    auto playbook_path = testing::getResourcePathFor("playbooks/basic_start_up.json");
    auto playbook = testing::Playbook(playbook_path);
    auto runner = testing::PlaybookRunner(std::move(playbook));

    // Load a Dummy JSON Delegate
    auto configure_path = testing::getResourcePathFor("configure/default.json");
    auto delegate = std::make_shared<testing::JSONConfiguredDelegate>(configure_path);
    runner.initializeWithDelegate(std::move(delegate));

    runner.run();
}