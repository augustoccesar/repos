package se.augustocesar.repos;

import jdk.graal.compiler.util.json.JsonParser;
import jdk.graal.compiler.util.json.JsonWriter;
import org.graalvm.collections.EconomicMap;

import java.io.File;
import java.io.IOException;
import java.io.StringWriter;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.time.Instant;
import java.time.LocalDate;
import java.time.ZoneId;
import java.util.List;
import java.util.Optional;
import java.util.logging.Logger;

public class GitHub {
    private static final Logger logger = Logger.getLogger(GitHub.class.getName());
    private static final String LATEST_RELEASE_PATH = "/repos/augustoccesar/repos/releases/latest";

    private final String host;
    private final HttpClient httpClient;

    public record Release(Version version, String downloadUrl) {
        public String toJson() {
            var writer = new StringWriter();
            try (var jsonBuilder = new JsonWriter(writer).objectBuilder()) {
                jsonBuilder.append("version", this.version.toString());
                jsonBuilder.append("download_url", this.downloadUrl);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }

            return writer.toString();
        }

        public static Release fromJson(final String json) {
            try {
                var jsonDict = JsonParser.parseDict(json);

                var version = Version.parse(jsonDict.get("version").toString());
                var downloadUrl = jsonDict.get("download_url").toString();

                return new Release(version, downloadUrl);
            } catch (IOException | Version.InvalidFormat e) {
                throw new RuntimeException(e);
            }
        }

        public static Release fromJson(final Path jsonFilePath) {
            try {
                return fromJson(Files.readString(jsonFilePath));
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        }
    }

    public GitHub() {
        this("https://api.github.com");
    }

    public GitHub(final String host) {
        this.host = host;
        this.httpClient = HttpClient.newHttpClient();
    }

    @SuppressWarnings("unchecked")
    public Optional<Release> fetchLatestRelease(OS os, Arch arch, boolean useCache) {
        if (useCache) return cachedLatestRelease(os, arch);

        try {
            var request = HttpRequest.newBuilder()
                    .uri(URI.create(this.host + LATEST_RELEASE_PATH))
                    .build();

            HttpResponse<String> response = this.httpClient.send(request, HttpResponse.BodyHandlers.ofString());
            if (response.statusCode() != 200) {
                logger.warning("Received non-200 status from GitHub: " + response.statusCode());

                return Optional.empty();
            }

            EconomicMap<String, Object> responseJson = JsonParser.parseDict(response.body());
            var tag = (String) responseJson.get("tag_name");
            var version = Version.parse(tag);

            var assets = (List<EconomicMap<String, Object>>) responseJson.get("assets");
            for (var asset : assets) {
                var assetName = (String) asset.get("name");

                if (assetName.contains(os.releaseName()) && assetName.contains(arch.releaseName())) {
                    var downloadUrl = (String) asset.get("browser_download_url");

                    return Optional.of(new Release(version, downloadUrl));
                }
            }

            return Optional.empty();
        } catch (IOException | InterruptedException | Version.InvalidFormat e) {
            logger.severe(e.toString());

            return Optional.empty();
        }
    }

    private Optional<Release> cachedLatestRelease(OS os, Arch arch) {
        File cacheFile = new File(Constants.UPDATE_CACHE_FILE_PATH);

        if (cacheFile.exists()) {
            Instant lastModified = Instant.ofEpochMilli(cacheFile.lastModified());
            Instant startOfDayInstant = LocalDate.now()
                    .atStartOfDay(ZoneId.systemDefault())
                    .toInstant();
            boolean cacheIsFromToday = lastModified.isAfter(startOfDayInstant);

            if (cacheIsFromToday) {
                return Optional.of(GitHub.Release.fromJson(cacheFile.toPath()));
            }
        }

        Optional<Release> release = fetchLatestRelease(os, arch, false);

        if (release.isEmpty()) {
            logger.warning("Failed to fetch latest release from GitHub.");

            return Optional.empty();
        }

        try {
            Files.writeString(
                    Path.of(Constants.UPDATE_CACHE_FILE_PATH),
                    release.get().toJson(),
                    StandardOpenOption.CREATE,
                    StandardOpenOption.TRUNCATE_EXISTING
            );
        } catch (IOException e) {
            logger.warning("Failed to write cache file: " + e);
        }

        return release;
    }
}
