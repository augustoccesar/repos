package se.augustocesar.repos.commands;

import picocli.CommandLine.Parameters;
import picocli.CommandLine.Command;
import se.augustocesar.repos.Shell;

import java.io.IOException;
import java.io.InputStream;
import java.util.concurrent.Callable;

@Command(name = "activate", mixinStandardHelpOptions = true, description = "Export helpers for shell.")
public class ActivateCommand implements Callable<Integer> {
    @Parameters(index = "0", description = "Name of the shell. One of: [${COMPLETION-CANDIDATES}]")
    Shell shell;

    @Override
    public Integer call() throws IOException {
        String fileName = String.format("/repos.%s", shell);

        try (InputStream stream = this.getClass().getResourceAsStream(fileName)) {
            if (stream != null) {
                byte[] bytes = stream.readAllBytes();
                String output = new String(bytes);

                System.out.println(output);

                return 0;
            } else {
                System.err.printf("Failed to read '%s' resource%n", fileName);

                return 1;
            }
        }
    }
}
