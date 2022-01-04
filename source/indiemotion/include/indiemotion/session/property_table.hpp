#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/session/property.hpp>
#include <indiemotion/session/property_observer.hpp>

namespace indiemotion
{
	class SessionPropertyTable
	{
	public:
		SessionPropertyTable()
		{
		}

		void set(SessionProperty&& property)
		{
			auto original = property.copy_empty();
			if(get(&original)) {
				if (original.value()->index() != property.value()->index())
				{
					std::stringstream msg;
					msg << "cannot set property, type does not match store type: ";
					msg << "sent: " + property.value_description();
					msg << " store: " + original.value_description();
					throw SessionPropertyTypeException(msg.str());
				}
			}
			_table.insert_or_assign(property.name(), std::move(property));
		}

		bool get(SessionProperty* property)
		{
			auto search = _table.find(property->name());
			if (search != _table.end())
			{
				property->_value_ptr = search->second._value_ptr;
				return property->_value_ptr != nullptr;
			}
			return false;
		}

	private:
		std::map<std::string, SessionProperty> _table;
	};
}