import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class RegexMatches02 {
  public static void test(String string1) {
    Pattern expression = Pattern.compile("W.*\\d[0-35-9]-\\d\\d-\\d\\d");

    Matcher matcher = expression.matcher(string1);

    while (matcher.find()) {
      System.out.println(matcher.group());
      String tmp = matcher.group();
      assert tmp.equals("WWWWW's Birthday is 12-17-77");
    }
  }
}
