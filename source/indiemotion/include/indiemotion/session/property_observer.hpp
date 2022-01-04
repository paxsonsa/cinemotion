#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/session/property.hpp>
#include <indiemotion/session/property_table.hpp>

namespace indiemotion
{
	struct PropertyObserverList
	{
		using Handler = std::function<void(const std::shared_ptr<SessionProperty::Value>)>;
		std::vector<std::pair<const SessionProperty, Handler>> observers;
		void update(const SessionProperty *property)
		{
			for (auto& observer: observers)
			{
				if (observer.first.name() == property->name())
					observer.second(property->value());
			}
		}
	};

	std::pair<const SessionProperty, PropertyObserverList::Handler> make_property_observer(SessionProperty property, PropertyObserverList::Handler handler)
	{
		return std::make_pair<const SessionProperty, PropertyObserverList::Handler>(
			std::move(property),
			std::move(handler)
		);
	}
}
