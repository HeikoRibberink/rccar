package nl.heikoribberink;

import java.io.BufferedOutputStream;
import java.io.IOException;
import java.util.Arrays;

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
		try {
			out.write(new byte[] { -1, 0 });
			out.flush();
		} catch (IOException e) {
			e.printStackTrace();
		}
		App.running = false;
		return false;
	}

	public boolean buttonUp(com.badlogic.gdx.controllers.Controller controller, int buttonCode) {
		// TODO Auto-generated method stub
		return false;
	}

	public boolean axisMoved(com.badlogic.gdx.controllers.Controller controller, int axisCode, float value) {
		// lx = 0, ly = 1, rx = 2, ry = 3;
		byte mode = 0;
		switch (axisCode) {
			case 1:
				mode = 1;
				break;
			case 2:
				mode = 2;
				break;
			default:
				return false;
		}
		try {
			byte[] b = new byte[] { mode, (byte) (-value * 127.0) };
			out.write(b);
			out.flush();
			System.out.println("Send: " + Arrays.toString(b));
		} catch (IOException e) {
			e.printStackTrace();
			App.running = false;
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
