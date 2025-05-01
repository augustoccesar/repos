package se.augustocesar.repos;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

import java.util.HashMap;

class RepositoryInfoTest {

    @Test
    void of() {
        HashMap<String, Object> indexes = new HashMap<>();
        indexes.put("0", "augustoccesar/repos");

        HashMap<String, Object> aliases = new HashMap<>();
        aliases.put("alias1", "@0");
        aliases.put("alias2", "adventofcode");
        aliases.put("alias3", "mentimeter/linkup");
        aliases.put("alias4", "github.com/rust-lang/rust");
        aliases.put("alias5", "git@gitlab.com:gitlab-org/gitlab.git");

        var config = new Config(
                "github.com",
                "augustoccesar",
                indexes,
                aliases
        );

        var result1 = RepositoryInfo.of(config, "alias1");
        assertEquals("github.com", result1.host());
        assertEquals("augustoccesar", result1.username());
        assertEquals("repos", result1.name());

        var result2 = RepositoryInfo.of(config, "alias2");
        assertEquals("github.com", result2.host());
        assertEquals("augustoccesar", result2.username());
        assertEquals("adventofcode", result2.name());

        var result3 = RepositoryInfo.of(config, "alias3");
        assertEquals("github.com", result3.host());
        assertEquals("mentimeter", result3.username());
        assertEquals("linkup", result3.name());

        var result4 = RepositoryInfo.of(config, "alias4");
        assertEquals("github.com", result4.host());
        assertEquals("rust-lang", result4.username());
        assertEquals("rust", result4.name());

        var result5 = RepositoryInfo.of(config, "alias5");
        assertEquals("gitlab.com", result5.host());
        assertEquals("gitlab-org", result5.username());
        assertEquals("gitlab", result5.name());
    }
}
