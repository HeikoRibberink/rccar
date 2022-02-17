package nl.heikoribberink;

import java.io.BufferedOutputStream;

import com.badlogic.gdx.controllers.ControllerListener;
import com.badlogic.gdx.controllers.PovDirection;
import com.badlogic.gdx.math.Vector3;

public class Controller implements ControllerListener {

	private BufferedOutputStream out;

	public Controller(BufferedOutputStream out) {
		this.out = out;
	}

	public void connected(com.badlogic.gdx.controllers.Controller controller) {
		// TODO Auto-generated method stub
	}

	public void disconnected(com.badlogic.gdx.controllers.Controller controller) {
		// TODO Auto-generated method stub
		
	}

	public boolean buttonDown(com.badlogic.gdx.controllers.Controller controller, int buttonCode) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean buttonUp(com.badlogic.gdx.controllers.Controller controller, int buttonCode) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean axisMoved(com.badlogic.gdx.controllers.Controller controller, int axisCode, float value) {
		//lx = 0, ly = 1, rx = 2, ry = 3;
		byte mode = 0;
		switch(axisCode) {
			case 1:
				mode |= 0x1;
			break;
			case 3:
				mode |= 0x2;
			break;
			default:
			break;
		}
		return false;
	}

	public boolean povMoved(com.badlogic.gdx.controllers.Controller controller, int povCode, PovDirection value) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean xSliderMoved(com.badlogic.gdx.controllers.Controller controller, int sliderCode, boolean value) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean ySliderMoved(com.badlogic.gdx.controllers.Controller controller, int sliderCode, boolean value) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean accelerometerMoved(com.badlogic.gdx.controllers.Controller controller, int accelerometerCode,
			Vector3 value) {
		// TODO Auto-generated method stub
		return false;
	}
	
}
