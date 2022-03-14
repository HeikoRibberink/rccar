package nl.heikoribberink;

import java.io.BufferedOutputStream;
import java.io.IOException;
import java.net.Socket;
import java.net.UnknownHostException;

import uk.co.electronstudio.sdl2gdx.SDL2ControllerManager;

import org.libsdl.SDL_Error;
public class App {

	private static final String HOST = "192.168.2.60";
	private static final int PORT = 8080;

	public static boolean running = true;

	public static void main(String[] args) throws UnknownHostException, IOException, SDL_Error {
		Socket s = new Socket(HOST, PORT);
		BufferedOutputStream out = new BufferedOutputStream(s.getOutputStream());
		SDL2ControllerManager cMgr = new SDL2ControllerManager();
		System.out.println("All is up!");
		cMgr.addListener(new nl.heikoribberink.Controller(out));
		while (running) {
			cMgr.pollState();
		}

		s.close();
	}
}
