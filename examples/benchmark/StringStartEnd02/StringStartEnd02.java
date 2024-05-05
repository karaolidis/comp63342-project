public class StringStartEnd02 {

    public static void test(String[] nondetStrings) {
        String[] strings = new String[4];
        strings[0] = nondetStrings[0];
        strings[1] = nondetStrings[1];
        strings[2] = nondetStrings[2];
        strings[3] = nondetStrings[3];

        int i = 0;
        for (String string : strings) {
            if (string.startsWith("te")) ++i;
        }
        assert i == 1;
    }
}
