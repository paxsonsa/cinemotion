#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/session/property.hpp>

namespace indiemotion
{
    class SessionPropertyTable
	{
	public:
		SessionPropertyTable() {}

		void set(SessionProperty &&property) {
			_table.insert_or_assign(property.name(), std::move(property));
		}

		bool get(SessionProperty *property) {
			auto search = _table.find(property->name());
			if (search != _table.end())
			{
				property->_value_ptr = search->second.value();
				return true;
			}
			return false;
		}

	private:
		std::map<std::string, SessionProperty> _table;
	};
}