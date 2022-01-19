#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/property_table.hpp>

namespace indiemotion
{
	struct EulerXForm
	{
		double tx;
		double ty;
		double tz;
		double rx;
		double ry;
		double rz;

		EulerXForm(): EulerXForm(0.0f) {}
		EulerXForm(double value): tx(value), ty(value), tz(value), rx(value), ry(value), rz(value) {}
		EulerXForm(double tx, double ty, double tz, double rx, double ry, double rz): tx(tx), ty(ty), tz(tz), rx(rx), ry(ry), rz(rz) {}

		bool operator==(const EulerXForm& xform)
		{
			return tx == xform.tx &&
			       ty == xform.ty &&
			       tz == xform.tz &&
			       rx == xform.rx &&
			       ry == xform.ry &&
			       rz == xform.rz;
		}

		bool operator==(EulerXForm& xform)
		{
			return tx == xform.tx &&
				ty == xform.ty &&
				tz == xform.tz &&
				rx == xform.rx &&
				ry == xform.ry &&
				rz == xform.rz;
		}
	};

	struct DeviceInfo
	{
		EulerXForm current_xform;
	};

	enum MotionCaptureMode {
		Idle,
		Live,
		Recording,
	};

	struct MotionCaptureInfo
	{
		MotionCaptureMode mode = MotionCaptureMode::Idle;
	};

	struct ContextView
	{
		std::shared_ptr<DeviceInfo const> const device_info() const
		{
			return _device_info;
		}

		std::shared_ptr<MotionCaptureInfo const> const mocap_info() const
		{
			return _mocap_info;
		}

		std::shared_ptr<PropertyTable const> property_table() const
		{
			return _property_table;
		}

	protected:
		std::shared_ptr<PropertyTable> _property_table;
		std::shared_ptr<DeviceInfo> _device_info;
		std::shared_ptr<MotionCaptureInfo> _mocap_info;
	};



	struct Context: ContextView {

		friend std::unique_ptr<Context> make_context();

		void property_table(std::unique_ptr<PropertyTable> table)
		{
			_property_table = std::move(table);
		}

		void device_info(std::unique_ptr<DeviceInfo> info)
		{
			_device_info = std::move(info);
		}

		void mocap_info(std::unique_ptr<MotionCaptureInfo> info)
		{
			_mocap_info = std::move(info);
		}

	};

	std::unique_ptr<Context> make_context()
	{
		auto c = std::make_unique<Context>();
		c->_property_table = std::make_unique<PropertyTable>();
		c->_device_info = std::make_unique<DeviceInfo>();
		c->_mocap_info = std::make_unique<MotionCaptureInfo>();
		return c;
	}
}