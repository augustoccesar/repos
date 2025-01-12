package se.augustocesar.repos;

import java.io.IOException;

public class Git {
    public static void clone(final String url, final String target) {
        try {
            var processBuilder = new ProcessBuilder("git", "clone", url, target).inheritIO();
            var process = processBuilder.start();

            process.waitFor();
        } catch (IOException e) {
            System.err.println("Failed to create process to clone repository.");
        } catch (InterruptedException e) {
            System.err.println("Interrupted clonning.");
        }
    }
}
