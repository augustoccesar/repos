package se.augustocesar.repos.commands;

import java.util.Optional;
import java.util.concurrent.Callable;

public interface Command extends Callable<Integer> {
    Optional<String> help();
}
