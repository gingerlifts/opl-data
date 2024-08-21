Sub CleanExcelFile()
    Dim ws As Worksheet
    Dim cell As Range
    Dim charIndex As Integer
    Dim newText As String
    Dim hasStrikethrough As Boolean
    
    ' Loop through each worksheet in the active workbook
    For Each ws In ActiveWorkbook.Sheets
        ' Loop through each cell in the used range of the worksheet
        For Each cell In ws.UsedRange
            newText = cell.Value
            hasStrikethrough = False
            
            ' Check each character in the cell for strikethrough
            For charIndex = 1 To Len(cell.Value)
                If cell.Characters(charIndex, 1).Font.Strikethrough Then
                    hasStrikethrough = True
                    Exit For
                End If
            Next charIndex
            
            ' If strikethrough is detected, prepend a hyphen to the cell value
            If hasStrikethrough Then
                newText = "-" & Replace(cell.Value, " ", "")
                cell.Value = newText
            End If
            
            ' Trim leading and trailing whitespace
            cell.Value = Trim(cell.Value)
        Next cell
        
        ' Remove all formatting after handling strikethrough
        ws.Cells.ClearFormats
    Next ws
    
    MsgBox "Formatting removed, strikethroughs handled, and whitespace trimmed!", vbInformation
End Sub
