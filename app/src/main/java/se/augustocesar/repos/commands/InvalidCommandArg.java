package se.augustocesar.repos.commands;

public class InvalidCommandArg extends Exception {
    public InvalidCommandArg(String message) {
        super(message);
    }
}
