package spiderbiggen.adventofcode.day3;

public class Main {
    public static void main(String args[]) {

        int input = 312051;

        int n = (int) Math.ceil(Math.sqrt(input)) + 1;
        int max = n * n;

        int[][] spiral = new int[n][n];

        int value = 0;

        int minCol = 0;
        int maxCol = n - 1;
        int minRow = 0;
        int maxRow = n - 1;

        while (value < n * n) {
            for (int i = minCol; i <= maxCol; i++) {
                spiral[minRow][i] = max - value;
                if (max - value == input) {
                    int y = Math.abs(minRow - n / 2);
                    int x = Math.abs(i - n / 2);
                    System.out.println(x + y);
                }
                value++;
            }

            for (int i = minRow + 1; i <= maxRow; i++) {
                spiral[i][maxCol] = max - value;
                if (max - value == input) {
                    int x = Math.abs(maxCol - n / 2);
                    int y = Math.abs(i - n / 2);
                    System.out.println(x + y);
                }
                value++;
            }

            for (int i = maxCol - 1; i >= minCol; i--) {
                spiral[maxRow][i] = max - value;
                if (max - value == input) {
                    int y = Math.abs(maxRow - n / 2);
                    int x = Math.abs(i - n / 2);
                    System.out.println(x + y);
                }
                value++;
            }

            for (int i = maxRow - 1; i >= minRow + 1; i--) {
                spiral[i][minCol] = max - value;
                if (max - value == input) {
                    int x = Math.abs(minCol - n / 2);
                    int y = Math.abs(i - n / 2);
                    System.out.println(x + y);
                }
                value++;
            }

            minCol++;
            minRow++;
            maxCol--;
            maxRow--;
        }

//        for (int i = 0; i < spiral.length; i++) {
//            for (int j = 0; j < spiral.length; j++) {
//                System.out.print(spiral[i][j] + "\t");
//            }
//
//            System.out.println();
//        }
    }
}
