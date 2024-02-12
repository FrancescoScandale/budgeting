package fs.budgeting;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

@RestController
public class RustController {

    @GetMapping("/budgeting")
    public String executeRustProgram() throws IOException, InterruptedException {
        //path to Rust executable
        String rustProgramPath = "scripts/target/debug/budgeting";
        
        //execute Rust program using ProcessBuilder
        System.out.println("DEBUG: starting rust executable");
        ProcessBuilder processBuilder = new ProcessBuilder(rustProgramPath);
        processBuilder.redirectErrorStream(true);

        Process process = processBuilder.start();
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));

        StringBuilder output = new StringBuilder();
        String line;
        while ((line = reader.readLine()) != null) {
            output.append(line).append("\n");
        }

        int exitCode = process.waitFor();
        if (exitCode == 0) {
            return "Rust program executed successfully:\n" + output.toString();
        } else {
            return "Error executing Rust program:\n" + output.toString();
        }
    }
}
