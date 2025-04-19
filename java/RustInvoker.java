import java.io.BufferedReader;
import java.io.InputStreamReader;

public class RustInvoker {
    public static void main(String[] args) {
        try {
            // Input strings to pass to the Rust program
            String input1 = "/Users/aradhyagopal/Rust/Resources";
            String input2 = "/Users/aradhyagopal/Rust/Output";

            // Create a ProcessBuilder to run the Rust executable
            ProcessBuilder processBuilder = new ProcessBuilder(
                "./video_merger", input1, input2
            );

            // Start the process
            Process process = processBuilder.start();

            // Read the output from Rust program
            BufferedReader reader = new BufferedReader(
                new InputStreamReader(process.getInputStream())
            );

            String line;
            while ((line = reader.readLine()) != null) {
                System.out.println(line);
            }

            // Wait for the process to finish
            int exitCode = process.waitFor();
            if (exitCode != 0) {
                System.err.println("Rust program exited with an error.");
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}