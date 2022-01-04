#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion
{
	struct SessionProperty
	{
		friend class SessionPropertyTable;
		using Value = std::variant<double, std::int64_t, std::string, bool>;

		static std::string index_to_type(const Value &v) {
			switch(v.index())
			{
			case 0:
				return "std::monotype";
			case 1:
				return "float";
			case 2:
				return "int64";
			case 3:
				return "string";
			case 4:
				return "bool";
			default:
				return "unknown";
			}
		}

		SessionProperty(std::string name) : _name(name)
		{
		}

		SessionProperty(std::string name, Value value) : _name(name)
		{
			_value_ptr = std::make_shared<Value>(std::move(value));
		}

		SessionProperty(const SessionProperty p, Value value) : SessionProperty(p.name(), value)
		{
		}

		std::string name() const
		{
			return _name;
		}

		const std::shared_ptr<Value> value() const
		{
			return _value_ptr;
		}

		std::optional<std::string> value_str() const
		{
			try
			{
				return value < std::string > ();
			}
			catch (std::bad_variant_access& exc)
			{
				return {};
			}
		}

		std::optional<std::int64_t> value_int64() const
		{
			try
			{
				return value < std::int64_t > ();
			}
			catch (std::bad_variant_access& exc)
			{
				return {};
			}
		}

		std::optional<double> value_double() const
		{
			try
			{
				return value < double > ();
			}
			catch (std::bad_variant_access& exc)
			{
				return {};
			}
		}

		std::optional<bool> value_bool() const
		{
			try
			{
				return value < bool > ();
			}
			catch (std::bad_variant_access& exc)
			{
				return {};
			}
		}

		template<typename T>
		bool contains() const
		{
			if (!_value_ptr)
				return false;
			return std::holds_alternative<T>(*_value_ptr);
		}

		bool has_value() const
		{
			return bool(_value_ptr);
		}

		SessionProperty copy_empty() const
		{
			return SessionProperty(name());
		}

		SessionProperty with_value(Value v) const
		{
			return SessionProperty(name(), v);
		}

		std::string value_description() {
			if (_value_ptr)
			{
				return index_to_type(*_value_ptr);
			}
			return "empty";
		}

	private:
		std::string _name;
		std::shared_ptr<Value> _value_ptr = nullptr;

		template<typename T>
		std::optional<T> value() const
		{
			if (_value_ptr)
			{
				return std::get<T>(*_value_ptr);
			}
			return {};
		}
	};
}