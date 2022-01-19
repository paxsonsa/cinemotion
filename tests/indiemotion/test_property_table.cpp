#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/property_table.hpp>

TEST_CASE("Test Property Lifecycle")
{
	auto table = indiemotion::PropertyTable();

	table.update({0, "name"}, "name1");
	table.update({1, "name"}, "name2");
	table.update({0, "number"}, 100);
	table.update({2, "boolean"}, true);

	auto value = table.find({0, "name"});
	REQUIRE("name1" ==  std::get<std::string>(*value));

	value = table.find({1, "name"});
	REQUIRE(std::get<std::string>(*value) == "name2");

	value = table.find({2, "name"});
	REQUIRE_FALSE(value);

	value = table.find({0, "number"});
	REQUIRE(std::get<int>(*value) == 100);

	value = table.find({2, "boolean"});
	value = table.find({2, "boolean"});
	REQUIRE(std::get<bool>(*value));
}

