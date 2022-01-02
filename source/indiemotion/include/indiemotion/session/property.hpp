//
// Created by Andrew Paxson on 2022-01-01.
//
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion
{
	struct SessionPropertyValue
	{
		enum ValueType
		{
			String = 0,
			Int,
			Float,
		};

		SessionPropertyValue(std::string v) : _value(v), _type(ValueType::String)
		{
		}

		SessionPropertyValue(int v) : _value(std::to_string(v)), _type(ValueType::Int)
		{
		}
		SessionPropertyValue(double v) : _value(std::to_string(v)), _type(ValueType::Float)
		{
		}

		std::string as_string() noexcept
		{
			return _value;
		}

		int as_int()
		{
			return std::stoi(_value);
		}

		double as_float()
		{
			return std::stod(_value);
		}

		ValueType type()
		{
			return _type;
		}

	private:
		ValueType _type;
		std::string _value;
	};

	struct SessionProperty
	{
		friend class SessionPropertyTable;

		SessionProperty(std::string name, SessionPropertyValue&& value) : _name(name)
		{
			_value_ptr = std::make_shared<SessionPropertyValue>(std::move(value));
		}

		SessionProperty(std::string name, std::shared_ptr<SessionPropertyValue> value) : _name(name)
		{
			_value_ptr = value;
		}

		SessionProperty(std::string name, int value) : _name(name)
		{
			_value_ptr = std::make_shared<SessionPropertyValue>(value);
		}

		SessionProperty(std::string name, double value) : _name(name)
		{
			_value_ptr = std::make_shared<SessionPropertyValue>(value);
		}

		SessionProperty(std::string name, std::string value) : _name(name)
		{
			_value_ptr = std::make_shared<SessionPropertyValue>(value);
		}

		SessionProperty(std::string name) : _name(name)
		{
		}

		std::string name()
		{
			return _name;
		}

		std::shared_ptr<SessionPropertyValue> value()
		{
			return _value_ptr;
		}

		bool is_empty() { return !_value_ptr; }

	private:
		std::string _name;
		std::shared_ptr<SessionPropertyValue> _value_ptr;

	};
}