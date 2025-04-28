package se.augustocesar.repos.commands;

import java.io.IOException;
import java.util.Optional;

public interface Command {
    Integer run(Appendable output) throws IOException;

    Optional<String> help();
}
