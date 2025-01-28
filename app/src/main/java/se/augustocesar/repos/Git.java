package se.augustocesar.repos;

import java.io.IOException;

public class Git {
    public static boolean clone(final String url, final String target) {
        try {
            var processBuilder = new ProcessBuilder("git", "clone", url, target).inheritIO();
            var process = processBuilder.start();

            return process.waitFor() == 0;
        } catch (IOException e) {
            System.err.println("Failed to create process to clone repository.");

            return false;
        } catch (InterruptedException e) {
            System.err.println("Interrupted clonning.");

            return false;
        }
    }
}
