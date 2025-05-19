package se.augustocesar.repos;

import com.github.tomakehurst.wiremock.junit5.WireMockRuntimeInfo;
import com.github.tomakehurst.wiremock.junit5.WireMockTest;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import static com.github.tomakehurst.wiremock.client.WireMock.*;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

@WireMockTest
class GitHubTest {

    @Test
    public void test_fetching_valid_latest_release(WireMockRuntimeInfo wmRuntimeInfo) throws IOException {
        String jsonResponse = new String(Files.readAllBytes(Paths.get("src/test/resources/github_latest_release.json")));

        stubFor(get("/repos/augustoccesar/repos/releases/latest")
                .willReturn(ok(jsonResponse))
        );

        var release = new GitHub(wmRuntimeInfo.getHttpBaseUrl()).fetchLatestRelease(OS.macOS, Arch.arm64);
        assertEquals("0.3.0", release.version().toString());
        assertEquals("https://github.com/augustoccesar/repos/releases/download/0.3.0/repos-0.3.0-aarch64-apple-darwin.tar.gz", release.downloadUrl());
    }

    @Test
    public void test_fetching_when_non_200(WireMockRuntimeInfo wmRuntimeInfo) throws IOException {
        stubFor(get("/repos/augustoccesar/repos/releases/latest")
                .willReturn(notFound())
        );

        var release = new GitHub(wmRuntimeInfo.getHttpBaseUrl()).fetchLatestRelease(OS.macOS, Arch.arm64);
        assertNull(release);
    }
}