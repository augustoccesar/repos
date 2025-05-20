package se.augustocesar.repos;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.*;

class VersionTest {

    @Test
    void test_parse_valid() throws Version.InvalidFormat {
        var input = "0.1.2";

        var version = Version.parse(input);

        assertEquals(0, version.major());
        assertEquals(1, version.minor());
        assertEquals(2, version.patch());
    }

    @ParameterizedTest
    @ValueSource(strings = {
            "0",
            "0.1",
            "0.-1.2",
            "0.1.2+cb4f76",
            "0.1.2-alpha.1"
    })
    void test_parse_with_invalid_throws(String input) {
        assertThrows(Version.InvalidFormat.class, () -> {
            Version.parse(input);
        });
    }

    @ParameterizedTest
    @CsvSource(
            value = {
                    "0.0.1:0.0.0",
                    "0.1.0:0.0.1",
                    "1.0.0:0.1.0",
                    "1.1.0:1.0.0",
            },
            delimiter = ':'
    )
    void test_isNewerThan(String left, String right) throws Version.InvalidFormat {
        var leftVersion = Version.parse(left);
        var rightVersion = Version.parse(right);

        assertTrue(leftVersion.isNewerThan(rightVersion));
    }

    @ParameterizedTest
    @CsvSource(
            value = {
                    "0.0.0:0.0.1",
                    "0.0.1:0.1.0",
                    "0.1.0:1.0.0",
                    "1.0.0:1.1.0",
            },
            delimiter = ':'
    )
    void test_isOlderThan(String left, String right) throws Version.InvalidFormat {
        var leftVersion = Version.parse(left);
        var rightVersion = Version.parse(right);

        assertTrue(leftVersion.isOlderThan(rightVersion));
    }

    @Test
    void test_toString() {
        var version = new Version(1, 2, 3);

        assertEquals("1.2.3", version.toString());
    }
}