package se.augustocesar.repos;

public enum Color {
    YELLOW("\u001B[33m");

    private final static String RESET = "\u001B[0m";

    private final String code;

    Color(String code) {
        this.code = code;
    }

    public String apply(String input) {
        return code + input + RESET;
    }
}
