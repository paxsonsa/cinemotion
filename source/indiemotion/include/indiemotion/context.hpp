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

		EulerXForm() : EulerXForm(0.0f)
		{
		}
		EulerXForm(double value) : tx(value), ty(value), tz(value), rx(value), ry(value), rz(value)
		{
		}
		EulerXForm(double tx, double ty, double tz, double rx, double ry, double rz)
			: tx(tx), ty(ty), tz(tz), rx(rx), ry(ry), rz(rz)
		{
		}

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

	struct DeviceDescriptor
	{
		EulerXForm current_xform;
	};

	struct CameraInfo
	{
		std::string name;
	};

	struct SceneDescriptor
	{
		std::string activeCameraName = "";
		std::vector<CameraInfo> cameras = {};
	};

	enum MotionCaptureMode
	{
		Idle,
		Live,
		Recording,
	};

	struct MotionCaptureDescriptor
	{
		MotionCaptureMode mode = MotionCaptureMode::Idle;
	};

	struct Context
	{
		std::shared_ptr<PropertyTable> property_table;
		std::shared_ptr<DeviceDescriptor> device;
		std::shared_ptr<MotionCaptureDescriptor> mocap;
		std::shared_ptr<SceneDescriptor> scene;
	};

	struct ContextView
	{
		ContextView(std::shared_ptr<Context> c) : _context(c)
		{

		}
		std::shared_ptr<DeviceDescriptor const> const device() const
		{
			return _context->device;
		}

		std::shared_ptr<MotionCaptureDescriptor const> const mocap() const
		{
			return _context->mocap;
		}

		std::shared_ptr<PropertyTable const> property_table() const
		{
			return _context->property_table;
		}

		std::shared_ptr<SceneDescriptor const> scene() const
		{
			return _context->scene;
		}

	protected:
		std::shared_ptr<Context> _context;
	};

	std::unique_ptr<Context> make_context()
	{
		auto c = std::make_unique<Context>();
		c->property_table = std::make_unique<PropertyTable>();
		c->device = std::make_unique<DeviceDescriptor>();
		c->mocap = std::make_unique<MotionCaptureDescriptor>();
		c->scene = std::make_unique<SceneDescriptor>();
		return c;
	}
}