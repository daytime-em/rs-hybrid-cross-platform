package com.example.jni_cmake

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
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
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.Center
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.compose.ui.unit.dp
import com.example.jni_cmake.ui.theme.RustAndroidProjectTheme
import com.example.jnilib.PrimeSieve
import com.example.jnilib.model.FoundPrimes
import com.patrykandpatrick.vico.compose.axis.horizontal.rememberBottomAxis
import com.patrykandpatrick.vico.compose.axis.vertical.rememberStartAxis
import com.patrykandpatrick.vico.compose.chart.Chart
import com.patrykandpatrick.vico.compose.chart.line.lineChart
import com.patrykandpatrick.vico.core.axis.formatter.AxisValueFormatter
import com.patrykandpatrick.vico.core.entry.entryModelOf
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
          val requestedNumber = remember { mutableStateOf<Int?>(null) }
          val primesResult = remember { mutableStateOf<CalculationResult?>(null) }
          val calculationScope = rememberCoroutineScope()

          DisposableEffect(key1 = requestedNumber.value) {
            onDispose {
              Log.d("MainActivity", "Disposing of primes result")
              val resultToDispose = primesResult.value
              // null-out this *before* calling release, or the recompose will cause a crash
              if (resultToDispose != null) {
                primesResult.value = null
                resultToDispose.primesResult.release()
              }
            }
          }

          Content(
            isLoading = isLoading.value,
            calcRes = primesResult.value,
            onCalculateRequested = { requestedNum ->
              isLoading.value = true
              calculationScope.launch(Dispatchers.Default) {
                Log.d("MainActivity", "Calculating primes up to $requestedNum")
                requestedNumber.value = requestedNum
                val primesData = PrimeSieve.evaluate(requestedNum)
                val densities = primeDensities(10, primesData.upTo, primesData.foundPrimes)
                Log.i(
                  "MainActivity", "Found ${"%,d".format(primesData.primeCount)} " +
                          "primes on [1,${"%,d".format(primesData.primeCount)}]"
                )
                Log.v(
                  "MainActivity",
                  "last few found primes: ${primesData.foundPrimes.takeLast(20)}"
                )
                MainScope().launch {
                  // native side needs its mem back. Copy now to avoid disposing later
                  primesResult.value = CalculationResult(primesData, densities)
                  isLoading.value = false
                }
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
  calcRes: CalculationResult?,
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
      Column() {
        RangeInput(
          inputStr = numberInput.value,
          onInputUpdated = { numberInput.value = it },
          onCalculateRequested = onCalculateRequested,
        )
        if (calcRes != null) {
          Spacer(modifier = modifier.size(16.dp))
          Text(
            "Found ${"%,d".format(calcRes.primesResult.primeCount)} primes between 1 and" +
                    " ${"%,d".format(calcRes.primesResult.upTo)}",
            fontSize = TextUnit(12F, TextUnitType.Sp),
            fontWeight = FontWeight.Medium,
          )
          Spacer(modifier = modifier.size(12.dp))
          PrimesChart(
            primeGroups = calcRes.primeGroups,
          )
        }
      }
    }
  }
}

@Composable
fun PrimesChart(
  primeGroups: List<Int>,
  modifier: Modifier = Modifier
) {
  Log.d("MainActivity", "found densities $primeGroups")
  val entryModel = entryModelOf(*primeGroups.toTypedArray())
  Chart(
    chart = lineChart(),
    model = entryModel,
    startAxis = rememberStartAxis(),
    bottomAxis = rememberBottomAxis(
      valueFormatter = { _, _ -> "" }
    ),
    modifier = modifier,
  )
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
    try {
      val valAsInt = (inputStr ?: "")
        .replace(",", "")
        .replace(" ", "")
        .toInt()//.replace(Regex("[. -_,]"), "").toInt()
      if (valAsInt > 0) {
        inputAsInt = valAsInt
      }
    } catch (_: NumberFormatException) {
    }

    TextField(
      value = inputStr ?: "",
      placeholder = { Text("Integer 1 <= n <= MAX_INT") },
      onValueChange = { newValue ->
        onInputUpdated(newValue)
      },
      keyboardOptions = KeyboardOptions(
        keyboardType = KeyboardType.Number,
        imeAction = ImeAction.Done,
      ),
      keyboardActions = KeyboardActions(
        onDone = { inputAsInt?.let { onCalculateRequested(it) } }
      ),
      modifier = Modifier
    )
    Spacer(modifier = Modifier.width(12.dp))
    Button(
      onClick = { inputAsInt?.let { onCalculateRequested(it) } },
      enabled = inputAsInt != null,
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
    RangeInput(inputStr = "1,i 000", onInputUpdated = {}, onCalculateRequested = {})
  }
}

@Preview(showBackground = true)
@Composable
fun ScreenContent() {
  RustAndroidProjectTheme {
    Content(isLoading = false, calcRes = null, onCalculateRequested = {})
  }
}

data class CalculationResult(
  val primesResult: FoundPrimes,
  val primeGroups: List<Int>
)
