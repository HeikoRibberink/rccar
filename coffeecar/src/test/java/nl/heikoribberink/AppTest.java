package nl.heikoribberink;

import org.libsdl.SDL;
import org.libsdl.SDL_GameController;

import uk.co.electronstudio.sdl2gdx.SDL2ControllerManager;

import com.badlogic.gdx.controllers.Controller;
import com.badlogic.gdx.controllers.ControllerListener;
import com.badlogic.gdx.controllers.PovDirection;
import com.badlogic.gdx.math.Vector3;

import org.junit.Test;

/**
 * Unit test for simple App.
 */
public class AppTest {
	public static void main(String[] args) throws InterruptedException {
		SDL2ControllerManager cMgr = new SDL2ControllerManager();
		
		cMgr.addListenerAndRunForConnectedControllers(new ControllerListener() {

			public void connected(Controller controller) {
				// TODO Auto-generated method stub

			}

			public void disconnected(Controller controller) {
				// TODO Auto-generated method stub

			}

			public boolean buttonDown(Controller controller, int buttonCode) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean buttonUp(Controller controller, int buttonCode) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean axisMoved(Controller controller, int axisCode, float value) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean povMoved(Controller controller, int povCode, PovDirection value) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean xSliderMoved(Controller controller, int sliderCode, boolean value) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean ySliderMoved(Controller controller, int sliderCode, boolean value) {
				// TODO Auto-generated method stub
				return false;
			}

			public boolean accelerometerMoved(Controller controller, int accelerometerCode, Vector3 value) {
				// TODO Auto-generated method stub
				return false;
			}

		});
	}
}
