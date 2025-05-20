package se.augustocesar.repos.commands;

import org.apache.commons.compress.archivers.tar.TarArchiveEntry;
import org.apache.commons.compress.archivers.tar.TarArchiveInputStream;
import org.apache.commons.compress.compressors.gzip.GzipCompressorInputStream;
import picocli.CommandLine;
import se.augustocesar.repos.Arch;
import se.augustocesar.repos.GitHub;
import se.augustocesar.repos.OS;
import se.augustocesar.repos.Version;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.net.URI;
import java.net.URL;
import java.nio.channels.Channels;
import java.nio.channels.ReadableByteChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.Optional;
import java.util.concurrent.Callable;
import java.util.logging.Logger;

@CommandLine.Command(
        name = "update",
        description = "Updates the CLI to the latest available version.",
        mixinStandardHelpOptions = true
)
public class UpdateCommand implements Callable<Integer> {
    private static final Logger logger = Logger.getLogger(UpdateCommand.class.getName());

    @CommandLine.ParentCommand
    private Repos reposCommand;

    @CommandLine.Option(
            names = {"--skip-cache"},
            description = "Force remote fetching of latest release by skipping the local cache"
    )
    boolean skipCache;

    @Override
    public Integer call() throws Exception {
        URL installationLocation = getClass()
                .getProtectionDomain()
                .getCodeSource()
                .getLocation();
        Version currentVersion = reposCommand.version();

        File executable = new File(installationLocation.toURI());
        if (executable.isDirectory()) {
            System.err.println("""
                    Current executable is a directory. This means that the current run is probably a development one.
                    Skipping update...
                    """);

            return 1;
        }

        Optional<GitHub.Release> latestRelease = this.reposCommand
                .github()
                .fetchLatestRelease(OS.fromSystem(), Arch.fromSystem(), !skipCache);

        if (latestRelease.isEmpty()) {
            System.out.println("Could not find a new release");

            return 0;
        }

        if (!currentVersion.isOlderThan(latestRelease.get().version())) {
            System.out.printf("Already on the latest release (%s)\n", currentVersion);

            return 0;
        }

        System.out.printf("Updating from %s to %s\n", currentVersion, latestRelease.get().version());

        var tempCompressedFile = File.createTempFile("repos-update", ".tar.gz");
        if (!downloadFile(latestRelease.get().downloadUrl(), tempCompressedFile.getPath())) {
            System.err.println("Failed to download new update");

            return 1;
        }

        Path decompressedPath = decompressFile(tempCompressedFile);
        if (decompressedPath == null) {
            System.err.println("Failed to decompress update");

            return 1;
        }

        Files.copy(decompressedPath, executable.toPath(), StandardCopyOption.REPLACE_EXISTING);

        System.out.println("Finished updating! 🎉");

        return 0;
    }

    private static Path decompressFile(File input) throws IOException {
        var tempUncompressedFile = File.createTempFile("repos", "");

        try (
                FileInputStream fis = new FileInputStream(input);
                GzipCompressorInputStream gis = new GzipCompressorInputStream(fis);
                TarArchiveInputStream tis = new TarArchiveInputStream(gis)
        ) {
            TarArchiveEntry entry;
            while ((entry = tis.getNextEntry()) != null) {
                if (!entry.isDirectory()) {
                    try (FileOutputStream fos = new FileOutputStream(tempUncompressedFile)) {
                        byte[] buffer = new byte[1024];
                        int len;
                        while ((len = tis.read(buffer)) > 0) {
                            fos.write(buffer, 0, len);
                        }
                    }

                    if (!tempUncompressedFile.setExecutable(true, false)) {
                        logger.severe("Failed to set new binary as executable.");

                        return null;
                    }

                    return tempUncompressedFile.toPath();
                }
            }
        }

        return null;
    }

    private static boolean downloadFile(String fileUrl, String savePath) {
        try {
            URL url = URI.create(fileUrl).toURL();
            try (
                    ReadableByteChannel readableByteChannel = Channels.newChannel(url.openStream());
                    FileOutputStream fileOutputStream = new FileOutputStream(savePath)
            ) {
                fileOutputStream.getChannel().transferFrom(readableByteChannel, 0, Long.MAX_VALUE);

                return true;
            }
        } catch (IOException e) {
            logger.severe("Failed to download file: " + e);

            return false;
        }
    }
}
