#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/controller.hpp>

using namespace indiemotion;

struct DummySceneDelegate: SceneDelegate
{
	std::optional<std::string> active_cam;

	std::vector<Camera> get_scene_cameras() override
	{
		return {
			Camera{"cam1"},
			Camera{"cam2"},
			Camera{"cam3"}
		};
	}

	void scene_updated(std::shared_ptr<SceneContext const> scene) override
	{
		active_cam = scene->active_camera_name;
	}
};

TEST_CASE("Test Scene Camera Manipulation")
{
	auto delegate = std::make_shared<DummySceneDelegate>();
	auto ctx = indiemotion::make_context();
	auto scene = indiemotion::SceneManager(ctx, delegate);
	REQUIRE(scene.cameras().size() == 3);

	scene.update_active_camera("cam1");
	REQUIRE(delegate->active_cam == "cam1");

	scene.update_active_camera({});
	REQUIRE_FALSE(delegate->active_cam.has_value());
}
