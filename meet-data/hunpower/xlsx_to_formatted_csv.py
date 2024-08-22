import pandas as pd
import os

def get_weight_class(bodyweight, is_male):
    """Determine the weight class based on bodyweight."""
    if is_male:
        weight_classes = {
            '53': 53.00,
            '57': 57.00,
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
            '74': 74.00,
            '83': 83.00,
            '83+': float('inf')  # '83+' means anything above 83.00
        }

    for weight_class, limit in weight_classes.items():
        if bodyweight <= limit:
            return weight_class
    return None

def assign_sex(weight_class, is_male):
    """Assign Sex based on weight class."""
    if is_male:
        return 'M'
    else:
        return 'W'

def process_excel_file(file_path):
    """Process the Excel file to clean data and assign weight classes."""
    # Determine if the file is male or female based on the file name
    file_name = os.path.basename(file_path).lower()
    if "fiu" in file_name or "ferfi" in file_name:
        is_male = True
    elif "noi" in file_name or "lany" in file_name:
        is_male = False
    else:
        raise ValueError("Cannot determine gender from file name: {}".format(file_name))

    # Load the Excel file and process all data as strings
    df = pd.read_excel(file_path, dtype=str)

    # 1. Remove all rows that precede the row with header entries
    header_index = df[df.eq('d.o.b.').any(axis=1)].index[0]
    df = df.iloc[header_index:]

    # Re-read the dataframe with the correct header
    df.columns = df.iloc[0]
    df = df.drop(df.index[0]).reset_index(drop=True)

    # 2. Remove all rows after the lifter data has ended
    end_index = df[df.eq('Team (points)').any(axis=1)].index[0]
    df = df.iloc[:end_index]

    # 3. Rename columns for consistency
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

    # 4. Remove rows where only the 'Place' column has data (likely weight class rows)
    df = df.dropna(subset=['Name', 'BirthYear', 'Team', 'BodyweightKg', 'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg'], how='all')

    # 5. Replace '-' or non-numeric values in 'Place' with 'DQ'
    df['Place'] = df['Place'].apply(lambda x: x if x.strip().isdigit() else 'DQ')

    # 6. Remove unnecessary columns
    df.drop(columns=['GL Coef', 'Lot', 'GL Pts', 'Pts'], inplace=True)

    # 7. Replace commas with dots in weight-related columns (but keep as strings)
    weight_columns = ['BodyweightKg', 'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg']
    for col in weight_columns:
        df[col] = df[col].str.replace(',', '.')

    # 8. Assign weight classes based on bodyweight and gender
    df['BodyweightKg'] = pd.to_numeric(df['BodyweightKg'], errors='coerce')
    df['WeightClassKg'] = df['BodyweightKg'].apply(lambda bw: get_weight_class(bw, is_male))

    # 9. Add Event, Equipment, and Sex columns
    df['Event'] = 'B'
    df['Equipment'] = 'Raw'
    df['Sex'] = df['WeightClassKg'].apply(lambda wc: assign_sex(wc, is_male))

    # Save the cleaned file with a new name
    output_file = file_path.replace('.xlsx', '_cleaned.xlsx')
    df.to_excel(output_file, index=False)
    print(f"Processed and saved: {output_file}")

if __name__ == "__main__":
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyv.xlsx")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyiv.xlsx")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/lanyvi.xlsx")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuv.xlsx")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuvi.xlsx")
    process_excel_file("/mnt/c/Users/aronhegedus/Downloads/hunpower2024-03-23/fiuiv.xlsx")
