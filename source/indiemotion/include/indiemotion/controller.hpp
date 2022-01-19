#pragma once
#include <indiemotion/common.hpp>

#include <indiemotion/context.hpp>
#include <indiemotion/context_manager.hpp>
#include <indiemotion/property_table.hpp>

namespace indiemotion
{
	struct IApplication
	{
		virtual void setup(std::shared_ptr<ContextManager> manager) = 0;
		virtual void on_update(const ContextView &context) = 0;
		virtual void on_property_update(const ContextView &context, const PropertyID& id, const PropertyValue& value) = 0;
	};

	struct Controller
	{
		std::shared_ptr<ContextManager> _manager;

		Controller() {
			auto context = Context::make_context();
			_manager = std::make_shared<ContextManager>(std::move(context));
		}



	};
}