package se.augustocesar.repos;

import jdk.graal.compiler.util.json.JsonParser;
import jdk.graal.compiler.util.json.JsonWriter;
import org.graalvm.collections.EconomicMap;

import java.io.IOException;
import java.io.StringWriter;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.List;
import java.util.logging.Logger;

public class GitHub {
    private static final Logger logger = Logger.getLogger(GitHub.class.getName());
    private static final String LATEST_RELEASE_PATH = "/repos/augustoccesar/repos/releases/latest";

    private final String host;
    private final HttpClient httpClient;

    public record Release(Version version, String downloadUrl) {
    }

    public GitHub() {
        this("https://api.github.com");
    }

    public GitHub(final String host) {
        this.host = host;
        this.httpClient = HttpClient.newHttpClient();
    }

    @SuppressWarnings("unchecked")
    public Release fetchLatestRelease(OS os, Arch arch) {
        try {
            var request = HttpRequest.newBuilder()
                    .uri(URI.create(this.host + LATEST_RELEASE_PATH))
                    .build();

            HttpResponse<String> response = this.httpClient.send(request, HttpResponse.BodyHandlers.ofString());
            if (response.statusCode() != 200) {
                logger.warning("Received non-200 status from GitHub: " + response.statusCode());

                return null;
            }

            EconomicMap<String, Object> responseJson = JsonParser.parseDict(response.body());
            var tag = (String) responseJson.get("tag_name");
            var version = Version.parse(tag);

            var assets = (List<EconomicMap<String, Object>>) responseJson.get("assets");
            for (var asset : assets) {
                var assetName = (String) asset.get("name");

                if (assetName.contains(os.releaseName()) && assetName.contains(arch.releaseName())) {
                    var downloadUrl = (String) asset.get("browser_download_url");

                    return new Release(version, downloadUrl);
                }
            }

            return null;
        } catch (IOException | InterruptedException | Version.InvalidFormat e) {
            logger.severe(e.toString());

            return null;
        }
    }
}
