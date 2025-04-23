package se.augustocesar.repos.commands;

import picocli.CommandLine.Command;
import se.augustocesar.repos.ReposDir;

import java.util.concurrent.Callable;

@Command(name = "list")
public class ListCommand implements Callable<Integer> {

    @Override
    public Integer call() throws Exception {
        var reposDir = ReposDir.load();

        for (var repo : reposDir.getRepos()) {
            System.out.println(repo);
        }

        System.out.println("List command");

        return 0;
    }
}
