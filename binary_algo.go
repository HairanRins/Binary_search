package main

import "fmt"

func rechercheBinaire(arr []int, cible int) int {
	gauche, droite := 0, len(arr)

	for gauche < droite {
		milieu := gauche + (droite-gauche)/2
		if arr[milieu] == cible {
			return milieu
		} else if arr[milieu] < cible {
			gauche = milieu + 1
		} else {
			droite = milieu
		}
	}
	return -1
}

func main() {
	arr := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	cible := 7
	index := rechercheBinaire(arr, cible)

	if index != -1 {
		fmt.Printf("Élément trouvé à l'indice: %d\n", index)
	} else {
		fmt.Println("Élément non trouvé dans le tableau")
	}
}
