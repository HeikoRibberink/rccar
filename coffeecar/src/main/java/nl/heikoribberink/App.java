package nl.heikoribberink;

import java.io.BufferedOutputStream;
import java.io.IOException;
import java.net.Socket;
import java.net.UnknownHostException;

import uk.co.electronstudio.sdl2gdx.SDL2ControllerManager;

import com.badlogic.gdx.controllers.Controller;
import com.badlogic.gdx.controllers.ControllerListener;
import com.badlogic.gdx.controllers.PovDirection;
import com.badlogic.gdx.math.Vector3;

import org.libsdl.SDL;
import org.libsdl.SDL_Error;
public class App 
{

	private static final String HOST = "192.168.2.60";
	private static final int PORT = 8080;

	private static boolean running = true;
	public static void main( String[] args ) throws UnknownHostException, IOException, SDL_Error
	{
		// Socket s = new Socket(HOST, 8080);
		// BufferedOutputStream bs = new BufferedOutputStream(s.getOutputStream());
		SDL2ControllerManager cMgr = new SDL2ControllerManager();
		System.out.println("All is up!");
		add(cMgr, null);
		while(true) {
			cMgr.pollState();
		}

		// s.close();
	}

	private static void add(SDL2ControllerManager cMgr, BufferedOutputStream out) {
		cMgr.addListener(new nl.heikoribberink.Controller(out));
	}
}
