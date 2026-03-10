import java.io.OutputStream;
import java.io.IOException;
import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.FileWriter;
import java.io.FileNotFoundException;
import java.io.File;
import java.io.BufferedReader;
import java.io.UnsupportedEncodingException;
import java.io.InputStreamReader;
import java.net.InetSocketAddress;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;
import java.util.stream.Collectors;

import com.sun.net.httpserver.HttpContext;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;

import com.sun.net.httpserver.HttpHandler;

public class Main {
    public static AlgoVMSP miner = null;

    public static void main(String[] args) throws IOException {
        miner = new AlgoVMSP();

        HttpServer server = HttpServer.create(new InetSocketAddress("localhost", 8001), 0);
        server.setExecutor(null);
        HttpContext context = server.createContext("/");
        context.setHandler(Main::handleRequest);
        server.start();
    }
    private static void handleRequest(HttpExchange exchange) throws IOException {
        Map<String, String> queryMap = getParamMap(exchange);

        double minsup = Double.parseDouble(queryMap.get("minsup"));
        String input = queryMap.get("input");

        inputStringToFile(input, "input$.txt");
        miner.runAlgorithm("input$.txt", "output$.txt", minsup);
        String response = outputFileToString("output$.txt");

        miner.printStatistics();

        exchange.sendResponseHeaders(200, response.getBytes().length);
        OutputStream os = exchange.getResponseBody();
        os.write(response.getBytes());
        os.close();
    }

    private static void inputStringToFile(String input, String fileName) throws IOException {
        FileWriter f = new FileWriter(fileName);
        List<String> lines = Stream.of(input.split("%0A"))
                .map(line -> line.trim())
                .collect(Collectors.toList());
        for (String line : lines) {
            line = line.replace("+", " ");
            f.write(line + "\n");
        }
        f.close();
    }

    private static String outputFileToString(String fileName) throws IOException, FileNotFoundException {
        FileInputStream fin = new FileInputStream(new File(fileName));
        BufferedReader reader = new BufferedReader(new InputStreamReader(fin));
        String line, result = "";
        while ((line = reader.readLine()) != null)
            result += line + "\n";
        reader.close();
        fin.close();
        return result;
    }

    private static Map<String, String> getParamMap(HttpExchange exchange) throws IOException, UnsupportedEncodingException {
        InputStreamReader isr = new InputStreamReader(exchange.getRequestBody(), "utf-8");
        BufferedReader br = new BufferedReader(isr);
        String query = br.lines().collect(Collectors.joining());

        br.close();
        isr.close();

        // query is null if not provided (e.g. localhost/path )
        // query is empty if '?' is supplied (e.g. localhost/path? )
        if (query == null || query.isEmpty())
            return Collections.emptyMap();

        return Stream.of(query.split("&"))
                .filter(s -> !s.isEmpty())
                .map(kv -> kv.split("=", 2))
                .collect(Collectors.toMap(x -> x[0], x -> x[1]));
    }
}
