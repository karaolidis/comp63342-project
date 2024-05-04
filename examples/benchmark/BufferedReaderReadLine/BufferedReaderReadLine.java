import java.io.BufferedReader;
import java.io.StringReader;

public class BufferedReaderReadLine {
  public static String check(BufferedReader br) throws Exception {
    String s = br.readLine();
    return s;
  }

  public static void test(String arg) {
    String thisLine;
    int numLines = 0;

    BufferedReader br = new BufferedReader(new StringReader(arg));
    try {
      while ((thisLine = check(br)) != null) {
        System.out.println(thisLine);
        numLines += 1;
      }
    } catch (Exception e) {
      assert false : "An unexpected exception occurred: " + e.getMessage();
    }

    assert numLines > 0;
  }
}
