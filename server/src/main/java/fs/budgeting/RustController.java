package fs.budgeting;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.multipart.MultipartFile;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

@Controller
public class RustController {

    @RequestMapping("/testingPath")
    public ResponseEntity<String> testingPath() {
        try {
            String workingDir = System.getProperty("user.dir") + "/scripts/files/report.csv";

            return ResponseEntity.ok("Working dir computed: " + workingDir);
        } catch (Exception e) {
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Failed to compute working dir");
        }
    }

    @RequestMapping("/test")
    public String test() {
        return "index.html";
    }

    @RequestMapping("/budgetInput")
    public ResponseEntity<String> handleFileUpload(@RequestParam("file") MultipartFile file) throws InterruptedException{
        try {
            if(file.isEmpty()){
                return ResponseEntity.ok("File is empty");
            }

            //compose target file path
            String targetFile = System.getProperty("user.dir") + "/scripts/files/report.csv";
            Path targetFilePath = Paths.get(targetFile);

            //copy file to target file path
            Files.copy(file.getInputStream(), targetFilePath, StandardCopyOption.REPLACE_EXISTING);
            String response = executeRustProgram();
            return ResponseEntity.ok(response);
        } catch (InterruptedException ie) {
            throw ie;
        } catch (Exception e) {
            e.printStackTrace();
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Failed to upload file");
        }
    }


    private String executeRustProgram() throws IOException, InterruptedException {
        //path to Rust executable
        //String rustProgramPath = "scripts/target/debug/budgeting";
        String rustProgramPath = "deliverables/budgeting";
        
        //execute Rust program using ProcessBuilder
        ProcessBuilder processBuilder = new ProcessBuilder(rustProgramPath);
        processBuilder.redirectErrorStream(true);

        Process process = processBuilder.start();
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));

        StringBuilder jsonBuilder = new StringBuilder();
        String line;
        while ((line = reader.readLine()) != null) {
            jsonBuilder.append(line);
        }

        int exitCode = process.waitFor();
        if (exitCode != 0) {
            System.out.println("RUST PROGRAM FAILED");
        }
        return jsonBuilder.toString();
    }
}
