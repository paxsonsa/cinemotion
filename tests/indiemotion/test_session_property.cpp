#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/errors.hpp>
#include <indiemotion/session/property.hpp>
#include <indiemotion/session/property_table.hpp>

using namespace indiemotion;

TEST_CASE("Test SessionCon Property")
{
	auto property = SessionProperty("name");
	REQUIRE_FALSE(property.has_value());
	REQUIRE_FALSE(property.value_int64());
	REQUIRE_FALSE(property.value_bool());
	REQUIRE_FALSE(property.value_double());
	REQUIRE_FALSE(property.value_str());

	property = SessionProperty("name", "string");
	REQUIRE(property.value_str() == "string");
	REQUIRE(property.contains<std::string>());
	REQUIRE(property.has_value());
	REQUIRE_FALSE(property.value_int64());
	REQUIRE_FALSE(property.value_bool());
	REQUIRE_FALSE(property.value_double());

	property = SessionProperty("name", 1.0f);
	REQUIRE(property.value_double() == 1.0f);
	REQUIRE(property.contains<double>());
	REQUIRE(property.has_value());
	REQUIRE_FALSE(property.value_int64());
	REQUIRE_FALSE(property.value_bool());
	REQUIRE_FALSE(property.value_str());

	property = SessionProperty("name", 1);
	REQUIRE(property.value_int64() == 1);
	REQUIRE(property.contains<std::int64_t>());
	REQUIRE(property.has_value());
	REQUIRE_FALSE(property.value_double());
	REQUIRE_FALSE(property.value_bool());
	REQUIRE_FALSE(property.value_str());
}

TEST_CASE("Test SessionCon Property Table")
{
	auto table = SessionPropertyTable();
	auto property = SessionProperty("name", "string");
	table.set(std::move(property));

	auto result = SessionProperty("name");
	REQUIRE(table.get(&result));
	REQUIRE(result.has_value());
	REQUIRE(result.contains<std::string>());
	REQUIRE(result.value_str() == "string");

	property = SessionProperty("name", 64);
	REQUIRE_THROWS_AS(table.set(std::move(property)), SessionPropertyTypeException);

	result = SessionProperty("doesnotexist");
	REQUIRE_FALSE(table.get(&result));
	REQUIRE_FALSE(result.has_value());
}