package fs.budgeting;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

@RestController
public class RustController {

    //@GetMapping("/budgeting")
    @GetMapping("/budgeting")
    public String executeRustProgram() throws IOException, InterruptedException {
        //path to Rust executable
        String rustProgramPath = "scripts/target/debug/budgeting";
        
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
        if (exitCode == 0) {
            return "Rust program executed successfully:\n" + jsonBuilder.toString();
        } else {
            return "Error executing Rust program:\n" + jsonBuilder.toString();
        }
    }

    // @RequestMapping("/test")
    // public String test() {
    //     return "index.html";
    // }

    // @RequestMapping("/budgetInput")
    // public ResponseEntity<String> handleFileUpload(@RequestParam("file") MultipartFile file) {
    //     try {
    //         // You can handle the file here, for example, save it to disk
    //         // For demonstration, let's just print the file name
    //         System.out.println("Received file: " + file.getOriginalFilename());
    //         return ResponseEntity.ok("File uploaded successfully");
    //     } catch (Exception e) {
    //         return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Failed to upload file");
    //     }
    // }

}
