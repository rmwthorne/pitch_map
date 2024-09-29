### README.md

# Pitch Converter

This is a small Rust command-line program for learning purposes. It converts a numeric pitch value (such as is used in karaoke software like [Ultrastar Deluxe](https://usdx.eu/format/)) into its corresponding musical note. The pitch values are based on MIDI note numbers, where the number `0` corresponds to the note **C4** (MIDI Note 60). 

## How It Works

- The program prompts the user to input a **Pitch** (as a number).
- It then converts the pitch number into its respective musical note (e.g., 0 = C4, 1 = C#4, 2 = D4, etc.).
- You can continue entering pitch values in a loop until you decide to quit by typing **`q`**.
- If a pitch value is outside the predefined range, a default message will be shown.

### Example Usage

```
Enter a Pitch (or q to quit):
0
0 -> C4

Enter a Pitch (or q to quit):
12
12 -> C5
```

### How to Run

1. Ensure you have [Rust installed](https://www.rust-lang.org/learn/get-started).
2. Clone the repository or copy the project files.
3. Open a terminal and navigate to the project folder.
4. Run the project with the following command:

   ```bash
   cargo run
   ```

5. Enter a pitch number, or type `q` to quit.

### Future Enhancements

- Add support for more pitches beyond the C4–C5 range.
- Improve the user interface with better error handling.
