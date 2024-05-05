import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class RegexMatches01 {
  public static void test(String inputString, String regex) {
    Pattern expression = Pattern.compile(regex);

    Matcher matcher = expression.matcher(inputString);

    while (matcher.find()) {
      System.out.println(matcher.group());
      String tmp = matcher.group();
      assert tmp.equals("WWWW's Birthday is 12-17-77");
    }
  }
}
