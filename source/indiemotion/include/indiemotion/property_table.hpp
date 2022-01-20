#pragma once
#include <string>
#include <unordered_map>
#include <variant>

namespace indiemotion
{

	enum PropertyScope {
		Global
	};

	const auto GP_MotionCaptureMode = "motion_capture_mode";
	const auto GP_ActiveCameraID = "active_camera";
	const auto GP_SessionName = "session_name";
	const auto GP_SessionStatus = "session_status";


	/**
	 * Represents a property id
	 */
	struct PropertyID
	{
		/// A scope is a useful int for allow duplicate names in different scopes.
		int scope;

		/// The name of the property
		std::string name;

		bool operator==(const PropertyID& pid) const
		{
			return scope == pid.scope && name == pid.name;
		}

		struct _hash_fn
		{
			std::size_t operator()(const PropertyID& pid) const
			{
				std::size_t h1 = std::hash<int>()(pid.scope);
				std::size_t h2 = std::hash<std::string>()(pid.name);
				return h1 ^ h2;
			}
		};
	};

	/**
	 * Defines an alias representing a property value as a variant of double, bool, string, or int.
	 */
	using PropertyValue = std::variant<double, bool, std::string, int>;

	/**
	 * A simple table for storing properties by an id and with a value.
	 *
	 * The table takes ownership of the id and value and wraps the value
	 * inside of a shared_ptr. Handles to the value are const (both the pointer and value).
	 *
	 * Usage:
	 * ```
	 * auto table = PropertyTable();
	 * table.update({0, "name"}, "value");
	 *
	 * auto value_ptr = table.find({0, "name"}); // value is populated
	 *
	 * ```
	 *
	 */
	struct PropertyTable
	{
		PropertyTable()
		{
		}

		/**
		 * Attempt to find some property by an id.
		 *
		 * If the value is found, then a shared_ptr is returned to the value.
		 * The pointer is nullptr when nothing is found.
		 *
		 * @param id A property id to look up.
		 * @return shared pointer, nullptr if the value is missing.
		 */
		std::shared_ptr<PropertyValue const> const find(const PropertyID &id) const
		{
			auto search = _table.find(id);
			if (search != _table.end())
			{
				return search->second;
			}
			return nullptr;
		}

		/**
		 * Update a property in the table.
		 *
		 * If the property is present the existing value is replaced or else a new entry is made.
		 *
		 * @param id A property id to use as the lookup key.
		 * @param value The variant value to store.
		 */
		void update(const PropertyID &&id, const PropertyValue &&value)
		{
			_table.insert_or_assign(std::move(id), std::make_shared<PropertyValue>(std::move(value)));
		}

	private:
		std::unordered_map<PropertyID, std::shared_ptr<PropertyValue>, PropertyID::_hash_fn> _table = {};

	};
}