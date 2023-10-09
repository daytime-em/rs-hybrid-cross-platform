package com.example.jni_cmake

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Done
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.Center
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.example.jni_cmake.ui.theme.RustAndroidProjectTheme
import com.example.jnilib.PrimeSieve
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.launch

class MainActivity2 : ComponentActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    setContent {
      RustAndroidProjectTheme {
        // A surface container using the 'background' color from the theme
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
          val isLoading = remember { mutableStateOf(false) }
          val calculationScope = rememberCoroutineScope()

          Content(
            isLoading = isLoading.value,
            onCalculateRequested = { requestedNum ->
              isLoading.value = true
              calculationScope.launch(Dispatchers.Default) {
                Log.d("MainActivity", "Calculating primes up to $requestedNum")
                val primesData = PrimeSieve.evaluate(requestedNum)
                Log.i("MainActivity", "Found ${primesData.primeCount} primes on [1,$requestedNum]")
                Log.v(
                  "MainActivity",
                  "last few found primes: ${primesData.foundPrimes.takeLast(20)}"
                )
                MainScope().launch { isLoading.value = false }
              }
            }
          )
        }
      }
    }
  }
}

@Composable
fun Content(
  isLoading: Boolean,
  onCalculateRequested: (Int) -> Unit,
  modifier: Modifier = Modifier
) {
  Box(
    contentAlignment = Center,
    modifier = modifier
      .fillMaxSize()
      .padding(all = 16.dp)
  ) {
    // Progress/Chart Area
    if (isLoading) {
      ProgressView()
    } else {
      val numberInput = remember { mutableStateOf<String?>(null) }
      RangeInput(
        inputStr = numberInput.value,
        onInputUpdated = { numberInput.value = it },
        onCalculateRequested = onCalculateRequested,
      )
    }
  }
}

@Composable
fun ProgressView(modifier: Modifier = Modifier) {
  Box(
    modifier = modifier
      .background(color = Color.Transparent, shape = RoundedCornerShape(12.dp))
      .border(
        width = 1.dp,
        color = Color.Gray,
        shape = RoundedCornerShape(12.dp),
      )
  ) {
    CircularProgressIndicator(
      modifier = Modifier
        .padding(all = 12.dp)
        .align(Alignment.Center)
        .size(48.dp),
    )
  }
}

@Composable
fun RangeInput(
  modifier: Modifier = Modifier,
  inputStr: String?,
  onInputUpdated: (String) -> Unit,
  onCalculateRequested: (Int) -> Unit,
) {
  Row(
    verticalAlignment = Alignment.CenterVertically,
    modifier = modifier
      .border(
        width = 1.dp,
        color = Color.Gray,
        shape = RoundedCornerShape(12.dp),
      )
      .padding(all = 12.dp)
  ) {
    var inputAsInt: Int? = null
    val calcEnabled = try {
      val valAsInt = (inputStr ?: "").toInt()
      inputAsInt = valAsInt
      valAsInt > 0
    } catch (e: NumberFormatException) {
      false
    }

    TextField(
      value = inputStr ?: "",
      placeholder = { Text("Integer 1 <= n <= MAX_INT") },
      onValueChange = { newValue ->
        onInputUpdated(newValue)
      },
      modifier = Modifier
    )
    Spacer(modifier = Modifier.width(12.dp))
    Button(
      onClick = { inputAsInt?.let { onCalculateRequested(it) } },
      enabled = calcEnabled,
      modifier = modifier
    ) {
      Icon(
        Icons.Filled.Done,
        contentDescription = "",
      )
    }
  }
}

@Preview(showBackground = true)
@Composable
fun ProgressPreview() {
  RustAndroidProjectTheme {
    ProgressView()
  }
}

@Preview(showBackground = true)
@Composable
fun InputPreview() {
  RustAndroidProjectTheme {
    RangeInput(inputStr = null, onInputUpdated = {}, onCalculateRequested = {})
  }
}

@Preview(showBackground = true)
@Composable
fun ScreenContent() {
  RustAndroidProjectTheme {
    Content(isLoading = false, onCalculateRequested = {})
  }
}
