#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/session/property_table.hpp>
#include <indiemotion/session/property.hpp>

using namespace indiemotion;

TEST_CASE("Test Session Property")
{
	auto property = SessionProperty("name");
	REQUIRE_FALSE(property.value());
	REQUIRE(property.is_empty());

	property = SessionProperty("name", "string");
	REQUIRE(property.value()->type() == SessionPropertyValue::ValueType::String);
	REQUIRE(property.value()->as_string() == "string");

	property = SessionProperty("name", 1.0f);
	REQUIRE(property.value()->type() == SessionPropertyValue::ValueType::Float);
	REQUIRE(property.value()->as_float() == 1.0f);

	property = SessionProperty("name", 1);
	REQUIRE(property.value()->type() == SessionPropertyValue::ValueType::Int);
	REQUIRE(property.value()->as_int() == 1);

	auto value = SessionPropertyValue("string");
	property = SessionProperty("name", std::move(value));
	REQUIRE(property.value()->type() == SessionPropertyValue::ValueType::String);
	REQUIRE(property.value()->as_string() == "string");
}

TEST_CASE("Test Session Property Table")
{
	auto table = SessionPropertyTable();
	auto property = SessionProperty("name", "string");
	table.set(std::move(property));

	auto result = SessionProperty("name");
	REQUIRE(table.get(&result));
	REQUIRE_FALSE(result.is_empty());
	REQUIRE(result.value()->type() == SessionPropertyValue::ValueType::String);
	REQUIRE(result.value()->as_string() == "string");

	result = SessionProperty("doesnotexist");
	REQUIRE_FALSE(table.get(&result));
	REQUIRE(result.is_empty());
}