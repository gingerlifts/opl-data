import pandas as pd
import os

def get_weight_class(bodyweight, is_male):
    """Determine the weight class based on bodyweight."""
    if is_male:
        weight_classes = {
            '53': 53.00,
            '59': 59.00,
            '66': 66.00,
            '74': 74.00,
            '83': 83.00,
            '93': 93.00,
            '105': 105.00,
            '120': 120.00,
            '120+': float('inf')  # '120+' means anything above 120.00
        }
    else:
        weight_classes = {
            '43': 43.00,
            '47': 47.00,
            '52': 52.00,
            '57': 57.00,
            '63': 63.00,
            '69': 69.00,
            '76': 76.00,
            '84': 84.00,
            '84+': float('inf')  # '84+' means anything above 84.00
        }

    for weight_class, limit in weight_classes.items():
        if bodyweight <= limit:
            return weight_class
    return None

def assign_sex(is_male):
    """Assign Sex based on the is_male flag."""
    return 'M' if is_male else 'F'

def process_excel_file(file_path, event_type, division):
    """Process the Excel file to clean data and assign weight classes."""
    # Determine if the file is male or female based on the file name
    file_name = os.path.basename(file_path).lower()
    is_male = "fiu" in file_name or "ferfi" in file_name

    # Load the Excel file and process all data as strings
    df = pd.read_excel(file_path, dtype=str)

    # 1. Remove all rows that precede the row with header entries
    header_index = df[df.eq('d.o.b.').any(axis=1)].index[0]
    df = df.iloc[header_index:]

    # Re-read the dataframe with the correct header
    df.columns = df.iloc[0]
    df = df.drop(df.index[0]).reset_index(drop=True)

    # 2. Remove all rows after the lifter data has ended

    # Step 1: Create a mask to identify rows that contain the word '(points)' (ignoring case)
    mask = df.apply(lambda col: col.str.contains(r"\(points\)", case=False, na=False)).any(axis=1)

    # Step 2: Get the index of the first row that matches the criteria
    end_index_candidates = df[mask].index

    # Step 3: If a match is found, trim the DataFrame
    if len(end_index_candidates) > 0:
        end_index = end_index_candidates[0]  # Get the first occurrence
        df = df.iloc[:end_index]  # Keep only the rows before this index
    else:
        print(f"Error: Could not find '(points)' in the file {file_path}.")
        exit(1)

    # 3. Rename columns based on the event type
    if event_type == 'Bench':
        df.rename(columns={
            'Rnk': 'Place',
            'Lifters': 'Name',
            'd.o.b.': 'BirthYear',
            'BWT': 'BodyweightKg',
            '1 Att.': 'Bench1Kg',
            '2 Att.': 'Bench2Kg',
            '3 Att.': 'Bench3Kg',
            'Result': 'Best3BenchKg'
        }, inplace=True)
        df['Event'] = 'B'
        df['TotalKg'] = df['Best3BenchKg'].replace('DSQ', '')  # Add TotalKg as Best3BenchKg
    elif event_type == 'Deadlift':
        df.rename(columns={
            'Rnk': 'Place',
            'Lifters': 'Name',
            'd.o.b.': 'BirthYear',
            'BWT': 'BodyweightKg',
            '1 Att.': 'Deadlift1Kg',
            '2 Att.': 'Deadlift2Kg',
            '3 Att.': 'Deadlift3Kg',
            'Result': 'Best3DeadliftKg'
        }, inplace=True)
        df['Event'] = 'D'
        df['TotalKg'] = df['Best3DeadliftKg'].replace('DSQ', '')  # Add TotalKg as Best3DeadliftKg
    elif event_type == 'FullPower':
        df.rename(columns={
            'Rnk': 'Place',
            'Lifters': 'Name',
            'd.o.b.': 'BirthYear',
            'BWT': 'BodyweightKg',
            'SQ': 'Best3SquatKg',
            'BP': 'Best3BenchKg',
            'DL': 'Best3DeadliftKg',
            'TOTAL': 'TotalKg'
        }, inplace=True)
        df['Event'] = 'SBD'
        # Replace DSQ in TotalKg with a blank cell
        df['TotalKg'] = df['TotalKg'].replace('DSQ', '')

    # 4. Handle DSQ cases for Best3BenchKg/Best3DeadliftKg
    if 'Best3BenchKg' in df.columns:
        df['Best3BenchKg'] = df['Best3BenchKg'].replace('DSQ', '')
    if 'Best3DeadliftKg' in df.columns:
        df['Best3DeadliftKg'] = df['Best3DeadliftKg'].replace('DSQ', '')

    # 5. Handle cases where lifters passed on an attempt (marked by "X")
    for col in ['Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg']:
        if col in df.columns:
            df[col] = df[col].replace('X', '')

    # 6. Replace "01.01." in the BirthYear column with "20"
    df['BirthYear'] = df['BirthYear'].str.replace("01.01.", "20")

    # 7. Remove rows where only the 'Place' column has data (likely weight class rows)
    columns_to_check = ['Name', 'BirthYear', 'Team', 'BodyweightKg', 'Best3BenchKg', 'Best3DeadliftKg', 'Best3SquatKg', 'TotalKg']
    df = df.dropna(subset=[col for col in columns_to_check if col in df.columns], how='all')

    # 8. Replace '-' or non-numeric values in 'Place' with 'DQ'
    df['Place'] = df['Place'].apply(lambda x: x if x.strip().isdigit() else 'DQ')

    # 9. Keep only the necessary columns
    keep_columns = ['Place', 'Name', 'BirthYear', 'Team', 'BodyweightKg', 'Bench1Kg', 'Bench2Kg', 'Bench3Kg',
                    'Best3BenchKg', 'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Best3DeadliftKg',
                    'Best3SquatKg', 'TotalKg', 'Event', 'Equipment', 'Sex', 'BirthDate', 'Division']
    df = df[[col for col in df.columns if col in keep_columns]]

    # Remove any columns with no headers or that were not explicitly kept
    df = df.loc[:, ~df.columns.str.contains('^Unnamed')]

    # 10. Replace commas with dots in weight-related columns (but keep as strings)
    weight_columns = ['BodyweightKg', 'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg',
                      'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Best3DeadliftKg',
                      'Best3SquatKg', 'TotalKg']
    for col in weight_columns:
        if col in df.columns:
            df[col] = df[col].str.replace(',', '.')

    # 11. Assign weight classes based on bodyweight and gender
    df['BodyweightKg'] = pd.to_numeric(df['BodyweightKg'], errors='coerce')
    df['WeightClassKg'] = df['BodyweightKg'].apply(lambda bw: get_weight_class(bw, is_male))

    # 12. Add Equipment, Sex, BirthDate, and Division columns
    df['Equipment'] = 'Raw'
    df['Sex'] = assign_sex(is_male)
    df['BirthDate'] = ''  # Add empty BirthDate column
    df['Division'] = division  # Set Division column based on the parameter

    # Save the cleaned file as a CSV file with a new name
    output_file = file_path.replace('.xlsx', '_cleaned.csv')
    df.to_csv(output_file, index=False, encoding='utf-8')
    print(f"Processed and saved: {output_file}")


if __name__ == "__main__":
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuiv.xlsx", "Bench", "Juniors")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuv.xlsx", "Bench", "Juniors")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuvi.xlsx", "Bench", "Juniors")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyiv.xlsx", "Bench", "Juniors")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyv.xlsx", "Bench", "Juniors")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyvi.xlsx", "Bench", "Juniors")
